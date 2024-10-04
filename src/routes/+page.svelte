<script lang="ts">
  // Import necessary modules from Svelte and Tauri
  import { onMount } from "svelte"; // Lifecycle function to run code when the component is mounted
  import { invoke } from "@tauri-apps/api/tauri"; // Function to invoke Tauri commands
  import { marked } from 'marked'; // Library for converting markdown text to HTML

  // Define a type alias for the possible statuses of the application
  type Status = "not checked" | "unreachable" | "ready" | "unavailable" | "unreachable application backend" | "generating";

  // State variables to track the application status and user input
  let nex_status: Status = "not checked"; // Current status of the application
  let prompt: string = ""; // User's input prompt
  let raw_user_output: string = ""; // Raw output received from the backend
  let user_output: string = ""; // Formatted user output to be displayed

  // Control variable to manage the state of output clearing
  let clearing: boolean = false;

  // Constants defining application behavior
  const prompt_max_length: number = 5000; // Maximum allowed length for the user prompt
  const statusColors: Record<Status, string> = { // Maps each status to a corresponding color
    "ready": "lightgreen",
    "generating": "#ffffff",
    "not checked": "#ffffff",
    "unavailable": "red",
    "unreachable application backend": "red",
    "unreachable": ""
  };
  
  const shadowQuantity: Record<Status, number> = { // Maps each status to a shadow size quantity
    "ready": 0.1,
    "generating": 0.2,
    "not checked": 0,
    "unavailable": 0,
    "unreachable application backend": 0,
    "unreachable": 0
  };

  // Reactive statements for automatic updates on data changes
  $: markdown_output = marked(user_output); // Convert user_output from markdown to HTML
  $: outputVisible = user_output.trim().length > 1; // Determine visibility of output based on content

  let logo: HTMLElement | null = null; // Reference to the logo element
  let nex_title: string = ''; // Title text to be displayed
  const title_letters: string = 'Welcome to Nex'; // Title string to be displayed incrementally
  let backend_conn: "not checked" | "unreachable" | "connected" = "not checked"; // Track backend connection status

  // Function to test the backend connection
  async function testBackend(): Promise<void> {
    backend_conn = "unreachable"; // Assume connection is unreachable initially
    try {
      console.log("Testing backend connection..."); // Log connection attempt
      const result = await invoke<string>("test_backend"); // Invoke backend command to test connection
      backend_conn = result === "connected" ? "connected" : "unreachable"; // Update connection status based on result
    } catch (error) {
      console.error("Backend connection test failed:", error); // Log any errors that occur
    }
  }

  // Function to log messages to the console via the backend
  async function log(level: number, message: string): Promise<void> {
    try {
      await invoke<string>("log", { level, message }); // Invoke backend command to log messages
    } catch (error) {
      console.error("Logging failed:", error); // Log any errors that occur
    }
  }

  // Function to check the AI status
  async function checkAIStatus(): Promise<void> {
    try {
      const response = await invoke<boolean>("ai_status"); // Invoke backend command to check AI status
      nex_status = response ? "ready" : "unavailable"; // Update status based on response
    } catch (error) {
      console.error("Failed to check AI status:", error); // Log any errors that occur
      nex_status = "unavailable"; // Set status to unavailable on error
    }
  }

  // Function to send the user's prompt to the backend
  async function sendPrompt(): Promise<void> {
    // If the backend is unreachable, update status and output
    if (backend_conn === "unreachable") {
      nex_status = "unreachable application backend"; // Update status
      user_output = "Please make sure you're using our latest Nex assistant version"; // Inform the user
      return; // Exit the function
    }

    nex_status = "generating"; // Update status to generating
    clearOutput(); // Clear previous output

    try {
      // Send prompt to backend and wait for the raw user output
      raw_user_output = await invoke<string>("send_prompt", { prompt });

      // Set an interval to append output gradually
      const titleInterval = setInterval(() => {
        appendOutput(); // Append output incrementally
        // If all output has been processed and not clearing, clear the interval and update status
        if (user_output.length >= raw_user_output.length && !clearing) {
          clearInterval(titleInterval); // Stop the interval
          nex_status = "ready"; // Update status to ready
        }
      }, 15); // Run every 15 milliseconds
    } catch (error) {
      // On error, update output and status
      user_output = "There was an error processing your request. Please try again."; // Inform the user
      nex_status = "unavailable"; // Set status to unavailable
      await log(2, "Failed to send prompt: " + error); // Log the error
    }
  }

  // Scheduler for logo shadow animation
  function scheduler(): number {
    let shadowSize = 0; // Initial shadow size
    let growing = true; // Control variable for growing/shrinking

    // Function to determine if animation should occur based on status
    function shouldAnimate(status: Status): boolean {
      return status === "ready" || status === "generating"; // Animate for specific statuses
    }

    // Function to update the logo shadow based on status
    function updateShadow(): void {
      const color = statusColors[nex_status]; // Get color for current status
      const animate = shouldAnimate(nex_status); // Check if animation is needed
      const rate = shadowQuantity[nex_status] || 0; // Get shadow rate for current status

      // Update shadow size based on whether it's growing or shrinking
      if (animate) {
        shadowSize = growing ? shadowSize + rate : shadowSize - rate; // Adjust size
        // Update growing state based on shadow size boundaries
        growing = shadowSize >= 3 ? false : shadowSize <= rate ? true : growing;
      } else {
        shadowSize = 1.5; // Reset shadow size if not animating
      }

      // Apply the shadow effect to the logo element if it exists
      if (logo) {
        logo.style.filter = `drop-shadow(0 0 ${shadowSize}em ${color})`; // Set drop-shadow effect
      }
    }

    return setInterval(updateShadow, 100); // Run updateShadow every 100 milliseconds
  }

  // Function to update the title text incrementally
  function updateTitle(): void {
    if (nex_title.length < title_letters.length) {
      nex_title += title_letters[nex_title.length]; // Append the next letter of the title
    }
  }

  // Function to clear output
  function clearOutput(): void {
    raw_user_output = ""; // Reset raw user output
    clearing = true; // Set clearing state to true
    // Set an interval to remove output gradually
    const clearIntervalId = setInterval(() => {
      removeOutput(); // Remove one character from output
      // If the user output is empty, stop clearing
      if (user_output.length <= 1) {
        clearInterval(clearIntervalId); // Stop the interval
        clearing = false; // Reset clearing state
      }
    }, 1); // Run every millisecond
  }

  // Function to append output incrementally
  function appendOutput(): void {
    if (!clearing && user_output.length < raw_user_output.length) {
      user_output += raw_user_output[user_output.length]; // Append the next character of raw output
    }
  }

  // Function to remove one character each iteration from user_output
  function removeOutput(): void {
    user_output = user_output.length > 2 ? user_output.slice(0, -1) : ""; // Remove the last character, or reset if too short
  }

  // Lifecycle method to initialize on component mount
  onMount(() => {
    scheduler(); // Start logo shadow animation
    setInterval(updateTitle, 100); // Start title update interval
    testBackend(); // Test backend connection
    checkAIStatus(); // Check AI status
  });
</script>

<!-- Svelte Component HTML -->
<div class="container">
  <h1>{nex_title}</h1>

  <div class="row">
    <a href="https://example.com" target="_blank">
      <img src="/nex.svg" bind:this={logo} class="logo nex" alt="Nex Logo" />
    </a>
  </div>

  <p>Nex AI status: <span aria-live="polite">{nex_status}</span></p>

  <form class="form-row" on:submit|preventDefault={sendPrompt}>
    <div class="char-counter-container">
      <span id="char-count" class="char-count">{prompt.length}/{prompt_max_length}</span>
    </div>
    <input 
      id="prompt-input" 
      placeholder="Enter your prompt..." 
      bind:value={prompt} 
      maxlength={prompt_max_length}
      aria-describedby="char-count"
    />
    <button type="submit" class:generating={nex_status === 'generating'} disabled={nex_status === 'generating'}>
      {#if nex_status === 'generating'}
        <span class="spinner"></span>
      {:else}
        Send
      {/if}
    </button>    
  </form>

  <!-- Output Container -->
  <div class="output" class:fade-out={!outputVisible}>
    <p>{@html markdown_output}</p>
  </div>    
</div>

<style>
  /* Styling and Responsive Design */
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    color: #0f0f0f;
    background-color: #f6f6f6;
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
    height: 100vh; /* Full viewport height */
  }

  .logo {
    height: 10em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 0.5s ease-in-out;
  }

  .row {
    display: flex;
    justify-content: center;
  }

  h1 {
    text-align: center;
  }

  .form-row {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 20px; /* Space between form and output */
  }

  .char-counter-container {
    display: flex;
    align-items: center;
    margin-right: 10px; /* Space between counter and input */
  }

  input,
  button {
    border-radius: 16px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s, width 0.5s, height 0.5s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
    position: relative;
    width: auto;
    height: auto;
  }

  button.generating {
    border-radius: 50%;
    width: 3em;
    height: 3em;
    padding: 0;
    line-height: 3em;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  button:disabled {
    cursor: not-allowed;
    opacity: 0.5;
  }

  button:hover {
    border-color: #396cd8;
  }

  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  #prompt-input {
    margin-right: 10px;
  }

  .char-count {
    font-size: 0.8em;
    color: rgb(93, 93, 93);
  }

  .output {
    text-align: left;
    margin-top: 20px;
    padding: 25px;
    border: 10px solid #ccc;
    border-radius: 16px;
    background-color: #fff;
    max-height: 30vh; /* Defined maximum height */
    max-width: auto;  /* Defined maximum width */
    overflow-y: auto;  /* Scroll when content exceeds the height */
    overflow-x: hidden; /* Hide horizontal overflow */
    transition: max-height 0.5s ease, width 0.5s ease, height 0.5s ease, opacity 0.5s ease; /* Smooth transition */
    width: fit-content;  /* Automatically adjust width */
    height: fit-content; /* Automatically adjust height */
  }

  .fade-out {
    max-height: 1px;
    width: 1px;
    opacity: 0;
    transition: none; /* Remove animation */
  }

  .spinner {
    border: 2px solid rgba(0, 0, 0, 0.1);
    border-left-color: #396cd8;
    border-radius: 50%;
    width: 1em;
    height: 1em;
    animation: spin 0.75s linear infinite;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }

    button:active {
      background-color: #0f0f0f69;
    }

    .output {
      background-color: #1f1f1f;
      border-color: #444;
    }
  }
</style>