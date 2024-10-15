---
title: Espionox
subtitle: A Rust crate for building LLM Applications
---

# Simplifying Writing LLM Applications in Rust

I began working on Espionox in August of 2023, it's an attempt to make
building Ai applications in Rust just as approachable as it is with other
libraries such as LangChain. 

---
# Features 
## Function Calling Syntax specific to Espionox
Instead of using raw JSON to define functions, they can be defined in a specific syntax I invented. I wrote a transpiler which converts functions written in this
syntax into a valid JSON request. For example: 

```
get_n_day_weather_forecast(location: string, format!: enum('celcius' | 'farenheight'), num_days!: integer)
    where 
        i am 'get an n-day weather forecast'
        location is 'the city and state, e.g. san francisco, ca'
        format is 'the temperature unit to use. infer this from the users location.'
        num_days is 'the number of days to forcast'
```

transpiles into:

```json
{
 "name": "get_n_day_weather_forecast",
      "description": "Get an N-day weather forecast",
      "parameters": {
        "type": "object",
        "properties": {
          "location": {
            "type": "string",
            "description": "the city and state, e.g. san francisco, ca"
          },
            "num_days": {
            "type": "integer",
            "description": "the number of days to forcast",
            },
          "format": {
            "type": "string",
            "enum": ["celcius", "fahrenheight"]
          }
        },
        "required": ["num_days", "format"]
    }
}
```

## Easily inject Callbacks into prompting flows
Espionox's callback system, called `Listeners` in the crate, make creating things like RAG pipelines and Self Consistency mechanisms a lot easier to build.

## Supported Providers
* OpenAi
* Anthropic
* Meta Ai (In development) 
* Local (In development) 
---

# Links
- [📦](https://crates.io/crates/espionox) Over 6000 downloads on crates.io 
- [![small-github-img](/public/assets/logos/github-mark-white.svg)](https://github.com/voidKandy/espionox/tree/master) Check out the code
