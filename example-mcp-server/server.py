from mcp.server.fastmcp import FastMCP
import logging

# Initialize the MCP server
mcp = FastMCP("example-server")

# Tool: Model-controlled action
@mcp.tool()
def get_weather_info(city: str, units: str = "celsius") -> str:
    """Get weather information for a city.
    
    Args:
        city: Name of the city
        units: Temperature units (celsius or fahrenheit)
    """
    # This is a mock example - in real use, you'd call a weather API
    return f"Weather in {city}: 22°{units[0].upper()}, Sunny"

@mcp.tool()
def calculate_temperature(celsius: float) -> dict:
    """Convert Celsius to Fahrenheit and Kelvin.
    
    Args:
        celsius: Temperature in Celsius
    """
    fahrenheit = (celsius * 9/5) + 32
    kelvin = celsius + 273.15
    return {
        "celsius": celsius,
        "fahrenheit": round(fahrenheit, 2),
        "kelvin": round(kelvin, 2)
    }

# Resource: Application-controlled data
@mcp.resource("config://weather")
def get_weather_config() -> str:
    """Return weather service configuration."""
    return "Weather API: Mock Service v1.0"

# Prompt: User-controlled template
@mcp.prompt()
def weather_report(location: str) -> str:
    """Create a weather report prompt template.
    
    Args:
        location: City or region name
    """
    return f"Please provide a detailed weather forecast for {location}, including temperature, conditions, and any weather alerts."

if __name__ == "__main__":
    # Use stdio transport for Claude Desktop/Kiro integration
    mcp.run(transport='stdio')
