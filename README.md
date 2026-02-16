# X-Gen - Experiment Generator

A CLI tool to generate experiment project structures with customizable templates. Perfect for researchers, data scientists, and developers who want consistent, well-structured experiment layouts.

## Features

- 🚀 **Quick Setup**: Generate experiment structures with a single command
- 🧩 **Template System**: Built-in and custom template support
- 📁 **Structured Layouts**: Predefined directory structures for different experiment types
- 🔧 **Extensible**: Easy to add new templates for different experiment types
- 🤖 **AI-Agent Ready**: Designed to work seamlessly with AI agents for automated experiment setup

## Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later)

### Install from Source
```bash
git clone https://github.com/your-username/x-gen.git
cd x-gen
cargo install --path .
```

### Install from Crates.io (when published)
```bash
cargo install x-gen
```

## Quick Start

Generate your first experiment:
```bash
x-gen my_experiment -d ./experiments -t basic
```

This creates:
- A directory structure based on the "basic" template
- A pre-filled README.md with sections for experiment planning
- Follow-up instructions for next steps

## Usage

### Basic Command
```bash
x-gen <EXPERIMENT_NAME> [OPTIONS]
```

### Options
- `-d, --destination <PATH>`: Destination directory (default: "./")
- `-t, --template <TEMPLATE>`: Template name (default: "basic")

### Examples
```bash
# Create experiment in current directory
x-gen my_research_project

# Create experiment in specific directory
x-gen ml_experiment -d ./projects

# Use a specific template
x-gen data_analysis -t basic
```

## Templates

X-Gen supports both built-in and custom templates:

### Built-in Templates
- `basic`: General machine learning experiment template

### Custom Templates
You can create your own templates by adding them to `~/.x-gen/templates/`:

1. Create a template directory: `~/.x-gen/templates/my_template/`
2. Add a template definition: `~/.x-gen/templates/my_template/my_template.toml`
3. Add supporting files: `~/.x-gen/templates/my_template/README.md`, `FOLLOW_UP_PROMPT.md`

#### Template Structure
The template definition file uses TOML format with ASCII tree notation:

```toml
[example]
name = "my_template"

structure = """
├── README.md              # Experiment documentation
├── src/                   # Source code
│   └── main.py
├── data/                  # Data files
│   └── input.csv
├── results/               # Results and outputs
│   └── output.json
└── notebooks/             # Jupyter notebooks
    └── analysis.ipynb
"""
```

## Extending X-Gen

### Adding New Templates

1. Create a new template directory in `~/.x-gen/templates/`
2. Define the structure in \\{template_name\}/\{template_name\}.toml`
3. Add custom README.md and FOLLOW_UP_PROMPT.md files as needed

### Template File Structure
Each template directory should contain:
- \\{name\}.toml`: Defines the directory structure using ASCII tree notation
- `README.md`: Template for the experiment's README file
- `FOLLOW_UP_PROMPT.md`: Instructions shown after experiment creation

### Example Custom Template
```bash
# Create a deep learning template
mkdir -p ~/.x-gen/templates/deep_learning
```

`~/.x-gen/templates/deep_learning/deep_learning.toml`:
```toml
[example]
name = "deep_learning"

structure = """
├── README.md
├── model/
│   ├── __init__.py
│   ├── architecture.py
│   └── trainer.py
├── data/
│   ├── __init__.py
│   ├── loader.py
│   └── preprocessing.py
├── experiments/
│   ├── config.yaml
│   └── run_experiment.py
├── results/
└── notebooks/
    └── exploratory_analysis.ipynb
""`
```

## Integration with AI Agents

X-Gen is designed to work well with AI agents:

1. Generate experiment structure: `x-gen my_experiment -t basic`
2. AI agent fills in the README.md with experiment details
3. AI agent implements the experiment based on the structured layout
4. Follow-up instructions guide the next steps

## Directory Structure

After installation, X-Gen creates:
``
~/.x-gen/
└── templates/           # Custom templates directory
    └── basic/          # Default template (auto-created)
``

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Support

For support, please open an issue on the GitHub repository.
