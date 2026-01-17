#!/usr/bin/env python3
"""
Generate golden test files from Jinja2 templates and JSON value files.
Usage:
    python generate_golden.py template.j2 values.json output.html
"""

import sys
import json
from jinja2 import Template

def main():
    if len(sys.argv) != 4:
        print("Usage: python generate_golden.py <template_file> <values_file> <output_file>")
        sys.exit(1)

    template_path, values_path, output_path = sys.argv[1], sys.argv[2], sys.argv[3]

    # Read template
    with open(template_path, 'r', encoding='utf-8') as f:
        template_content = f.read()

    # Read values
    with open(values_path, 'r', encoding='utf-8') as f:
        values = json.load(f)

    # Render
    template = Template(template_content)
    rendered = template.render(**values)

    # Write output
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(rendered)

    print(f"Generated: {output_path}")

if __name__ == "__main__":
    main()
