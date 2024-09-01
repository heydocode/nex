# Nex Virtual Assistant

Nex is a personal project aimed at creating a cool UI with AI support. This project allows you to interact with a virtual assistant powered by AI using the Ollama API and the `ollama-rs` library.

## Features

- **Virtual Assistant**: Interact with an AI-powered assistant that can help you with various tasks.
- **Customizable Model**: Choose your AI model and even customize its identity and behavior.
- **User-Friendly UI**: Built with a clean and intuitive interface.
- **Tauri Framework**: Developed using Tauri for a lightweight and secure desktop application.

## Future Improvements
<details>
    <summary>Click here to toggle on/off future improvements/tasks</summary>

We're constantly working to make Nex Assistant more powerful, user-friendly, and feature-rich. Below is a list of planned improvements that will enhance both the functionality and overall experience of the project.

### 🗣️ Chat Speech & Speech-to-Text

- **Voice Interaction**: Implement speech recognition and synthesis to allow users to interact with the virtual assistant using voice commands. This feature will make Nex Assistant more accessible and provide a more natural way to communicate.
- **Multilingual Support**: Expand speech-to-text and text-to-speech capabilities to support multiple languages, catering to a broader range of users globally.

### 💬 Enhanced Conversational Abilities

- **Contextual Awareness**: Improve the assistant’s ability to maintain context across multiple interactions, allowing for more natural and coherent conversations.
- **Memory Features**: Enable the assistant to remember user preferences, previous interactions, and personalized data to provide a more tailored experience.

### 🛠️ Internal Functionality & Architecture Enhancements

- **Modular Plugin System**: Develop a plugin architecture that allows users to add or remove features as needed, making the assistant highly customizable and extendable.
- **Improved Error Handling**: Further refine the error handling mechanisms, making the system more robust and capable of gracefully managing unexpected scenarios.
- **Performance Optimization**: Continue optimizing the codebase for speed and efficiency, reducing the application’s memory footprint and startup time.
- **Advanced Configuration Options**: Provide more detailed configuration settings, allowing advanced users to fine-tune the assistant’s behavior and performance.

### 🔍 AI Model & Backend Enhancements

- **Dynamic Model Loading**: Implement a feature that allows users to switch between different AI models on the fly without restarting the application.
- **AI Model Training**: Introduce basic tools for users to train custom AI models within the Nex Assistant environment, enhancing personalization and relevance.
- **Backend Refactoring**: Restructure the backend to improve scalability and support future features like real-time collaboration and integration with external APIs.

### 🎨 UI/UX Improvements

- **Dark Mode**: Add a dark mode option for users who prefer a different aesthetic or work in low-light environments.
- **Customizable Themes**: Allow users to personalize the look and feel of the interface with customizable themes and layouts.
- **Responsive Design**: Ensure the application is fully responsive and works seamlessly across different screen sizes, including mobile devices.

### 🛡️ Security & Privacy

- **Data Encryption**: Implement end-to-end encryption for all user interactions and data storage, ensuring the highest level of privacy.
- **User Anonymization**: Provide options for users to anonymize their data, giving them control over what information is stored and processed.

### 🚀 Deployment & Distribution

- **Cross-Platform Support**: Expand support for additional operating systems beyond the current platforms, including Linux and potentially mobile OSes like Android and iOS.
- **Easy Installer**: Create a user-friendly installer package that simplifies the setup process, making it easier for non-technical users to get started.

These improvements are just the beginning. We are committed to continuously evolving Nex Assistant to meet the needs and expectations of our users. Stay tuned for more updates, and as always, your feedback and contributions are welcome!

</details>

## Screenshots

<details>
    <summary>Click here to toggle on/off the screenshots' section</summary>
    
First status: not checked
    
Explanation: the application's script has not checked the status yet.
    
![image](https://github.com/heydocode/Nex/blob/main/github_resources/screenshots/first_status_not_checked.png)
    
Second status: ready
    
Exlanation: ollama has responded to the backend ping by a list of available AI models. This happens when `nex:latest` is in this list.
    
![image](https://github.com/heydocode/Nex/blob/main/github_resources/screenshots/second_status_ready.png)

Third status: generating

Explanation: the prompt has been sended to the AI  model. The prompt is processing and the user have to wait until the response won't be received from ollama.

![image](https://github.com/heydocode/Nex/blob/main/github_resources/screenshots/third_status_generating.png)

Fourth status: unavailable

Explanation: 2 possibilities. The first one is "ollama is not running", another one is "nex is not in the list of available models".

![image](https://github.com/heydocode/Nex/blob/main/github_resources/screenshots/fourth_status_unavailable.png)
    
Fifth status: unreachable application backend
    
Explanation: the application's script detects that the backend don't respond to the client.
    
![image](https://github.com/heydocode/Nex/blob/main/github_resources/screenshots/fifth_status_unreachable_application_backend.png)

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
This project is licensed under the Apache License 2.0. See the LICENSE file for details.
```
