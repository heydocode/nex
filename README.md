# Nex Virtual Assistant

Nex is a personal project aimed at creating a cool UI with AI support. This project allows you to interact with a virtual assistant powered by AI using the Ollama API and the `ollama-rs` library.

## Features

- **Virtual Assistant**: Interact with an AI-powered assistant that can help you with various tasks.
- **Customizable Model**: Choose your AI model and even customize its identity and behavior.
- **User-Friendly UI**: Built with a clean and intuitive interface.
- **Tauri Framework**: Developed using Tauri for a lightweight and secure desktop application.

## Screenshots

<details>
    <summary>Click here to toggle on/off the screenshots' section</summary>
    
First status: not checked
    
Explanation: the application's script has not checked the status yet.
    
![image](https://github.com/heydocode/Nex/blob/main/screenshots/first_status_not_checked.png)
    
Second status: ready
    
Exlanation: ollama has responded to the backend ping by a list of available AI models. This happens when `nex:latest` is in this list.
    
![image](https://github.com/heydocode/Nex/blob/main/screenshots/second_status_ready.png)

Third status: generating

Explanation: the prompt has been sended to the AI  model. The prompt is processing and the user have to wait until the response won't be received from ollama.

![image](https://github.com/heydocode/Nex/blob/main/screenshots/third_status_generating.png)

Fourth status: unavailable

Explanation: 2 possibilities. The first one is "ollama is not running", another one is "nex is not in the list of available models".

![image](https://github.com/heydocode/Nex/blob/main/screenshots/fourth_status_unavailable.png)
    
Fifth status: unreachable application backend
    
Explanation: the application's script detects that the backend don't respond to the client.
    
![image](https://github.com/heydocode/Nex/blob/main/screenshots/fifth_status_unreachable_application_backend.png)

</details>

## Getting Started

Follow these steps to set up and run the Nex virtual assistant on your machine.

### Prerequisites

- [Ollama](https://ollama.com/download) installed on your machine.
- [Node.js](https://nodejs.org/) and [npm](https://www.npmjs.com/get-npm) installed.
- A text generation AI model (e.g., `gemma2:2B`).
- **Important: recommended 8B+ params models**

### Installation

1. Clone this repository:
    ```bash
    git clone https://github.com/heydocode/Nex.git
    cd Nex
    ```

2. Download and install Ollama by following the [instructions](https://ollama.com/download).

3. Run the AI model (e.g., `gemma2:2B`):
    ```bash
    ollama run gemma2:2B
    ```

4. Set up your virtual assistant. If you want the model to identify itself as "Alex," run:
    ```bash
    /set system you are a virtual assistant: Alex, your task is to help the user
    ```

5. Save your configuration:
    ```bash
    /save nex
    ```

### Running the Project

Once you've set up the AI, you can build and run the project using Tauri:

```bash
npm run tauri dev
```

If you don't have npm installed, refer to the Tauri documentation for further instructions.
Enjoy Your Virtual Assistant!

You should now have the Nex virtual assistant up and running. Start interacting with your AI and customize it to fit your needs.

## Issues

If you encounter any problems, feel free to open an issue in this repository. I'm here to help!

## Changelogs

<details>
    <summary>Click here to toggle on/off the changelogs' section</summary>
    <details>
        <summary>Nex v0.1.0 >> v0.2.0</summary>
        
### Output field, typescript, and more!

- new output field
- added full typescript support
- all code has been rewritten in typescript
- error handlers have been enhanced & corrected
- added an input limit with configuration (limit = constant)
- added MarkDown format to output
- fixed all bugs & lags in the application
- added a constant in rust side to choose the AI model to avoid misconfiguration
        
    </details>
</details>

## License
```
This project is licensed under the MIT License. See the LICENSE file for details.
```
