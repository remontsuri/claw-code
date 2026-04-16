# Example MCP Server

A simple MCP server demonstrating the three core primitives: Tools, Resources, and Prompts.

## Setup

```bash
# Install dependencies
uv sync

# Activate virtual environment
source .venv/bin/activate  # On Windows: .venv\Scripts\activate
```

## Testing

```bash
# Test with MCP Inspector (interactive UI)
uv run mcp dev server.py

# Or run directly
uv run python server.py
```

## Install in Kiro

Add to your `~/.kiro/settings/mcp.json`:

```json
{
  "mcpServers": {
    "example-server": {
      "command": "uv",
      "args": [
        "--directory",
        "/absolute/path/to/example-mcp-server",
        "run",
        "server.py"
      ]
    }
  }
}
```

## What's Included

- **Tools**: `get_weather_info`, `calculate_temperature` - Actions the AI can call
- **Resource**: `config://weather` - Read-only configuration data
- **Prompt**: `weather_report` - Reusable prompt template
