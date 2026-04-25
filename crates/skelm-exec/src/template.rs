use minijinja::context;

fn raise_exception(err_text: String) -> Result<String, minijinja::Error> {
    Err(minijinja::Error::new(
        minijinja::ErrorKind::SyntaxError,
        err_text,
    ))
}

fn strftime_now(s: String) -> Result<String, minijinja::Error> {
    if s == "%Y-%m-%d" {
        let naive = chrono::Utc::now().naive_utc();
        Ok(format!("{}", naive))
    } else {
        Err(minijinja::Error::new(
            minijinja::ErrorKind::InvalidOperation,
            format!("invalid format {}", s),
        ))
    }
}

pub fn chat_template(template: &str, system: &str, prompt: &str) -> Result<String, String> {
    chat_template_with_tools(template, system, prompt, &[])
}

pub fn chat_template_with_tools(template: &str, system: &str, prompt: &str, tools: &[crate::ToolDef]) -> Result<String, String> {
    let mut env = minijinja::Environment::new();
    minijinja_contrib::add_to_environment(&mut env);

    env.set_unknown_method_callback(minijinja_contrib::pycompat::unknown_method_callback);
    env.add_function("raise_exception", raise_exception);
    env.add_function("strftime_now", strftime_now);

    const MAIN: &str = "main";

    env.add_template(MAIN, template).unwrap();

    let tmpl = env.get_template(MAIN).unwrap();
    let messages = vec![
        context!(role => "system", content => system),
        context!(role => "user", content => prompt),
    ];

    // Convert tools to template-compatible format.
    // Provide both flat keys (Qwen) and OpenAI-style function wrapper (Gemma 4, Llama 3.x)
    // so the model's embedded Jinja template finds whichever format it expects.
    let tool_values: Vec<minijinja::Value> = tools.iter().map(|t| {
        let make_params = || {
            let props: std::collections::BTreeMap<String, minijinja::Value> = t.parameters.iter().map(|p| {
                (p.name.clone(), context!(
                    type => p.param_type.clone(),
                    description => p.name.clone(),
                    required => p.required
                ).into())
            }).collect();
            context!(type => "object", properties => props)
        };
        let function_block = context!(
            name => t.name.clone(),
            description => t.description.clone(),
            parameters => make_params()
        );
        context!(
            type => "function",
            function => function_block,
            name => t.name.clone(),
            description => t.description.clone(),
            parameters => make_params()
        ).into()
    }).collect();

    tmpl.render(context!(tools => tool_values, messages => messages, enable_thinking => false, add_generation_prompt => true))
        .map_err(|e| format!("chat template error {}", e))
}
