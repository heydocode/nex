<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { marked } from 'marked';

  // Types
  type Status = "not checked" | "unreachable" | "ready" | "unavailable" | "unreachable application backend" | "generating";
  
  // State variables
  let nex_status: Status = "not checked";
  let prompt: string = "";
  let raw_user_output: string = "";
  let user_output: string = "";

  // Control variable for output clearing
  let clearing: boolean = false;

  // Constants
  const prompt_max_length: number = 5000; // Limit the prompt length

  // Reactive statements
  $: markdown_output = marked(user_output);
  $: outputVisible = user_output.trim().length > 1;

  let logo: HTMLElement | null = null;
  let nex_title: string = '';
  const title_letters: string = 'Welcome to Nex';

  let backend_conn: "not checked" | "unreachable" | "connected" = "not checked";

  // Test the backend connection
  async function testBackend(): Promise<void> {
    try {
      console.log("Testing backend connection...");
      backend_conn = "unreachable";
      const result = await invoke<string>("test_backend");
      console.log("Backend test result:", result);
      backend_conn = result === "connected" ? "connected" : "unreachable";
    } catch (error) {
      console.error("Backend connection test failed:", error);
      backend_conn = "unreachable";
    }
  }

  // Print log in console
  async function log(level: number, message: string): Promise<void> {
    try {
      await invoke<string>("log", { level, message });
    } catch (error) {
      console.error("Logging failed:", error);
    }
  }

  // Check AI status
  async function checkAIStatus(): Promise<void> {
    try {
      const response = await invoke<boolean>("ai_status");
      nex_status = response ? "ready" : "unavailable";
    } catch (error) {
      console.error("Failed to check AI status:", error);
      nex_status = "unavailable";
    }
  }

  async function sendPrompt(): Promise<void> {
    if (backend_conn === "unreachable") {
      nex_status = "unreachable application backend";
      user_output = "Please make sure you're using our latest Nex assistant version";
      return;
    }

    nex_status = "generating";
    clearOutput();

    try {
      raw_user_output = await invoke<string>("send_prompt", { prompt });

      const titleInterval = setInterval(() => {
        appendOutput();
        if (user_output.length >= raw_user_output.length && !clearing) {
          clearInterval(titleInterval);
          nex_status = "ready";
        }
      }, 15);
    } catch (error) {
      user_output = "There was an error processing your request. Please try again.";
      nex_status = "unavailable";
      await log(2, "Failed to send prompt: " + error);
    }
  }

  // Scheduler for logo shadow animation
  function scheduler(): number {
    let shadowSize = 0;
    let growing = true;

    const statusColors: Record<Status, string> = {
      "ready": "lightgreen",
      "generating": "#ffffff",
      "not checked": "#ffffff",
      "unavailable": "red",
      "unreachable application backend": "red",
      "unreachable": ""
    };

    const shadowQuantity: Record<Status, number> = {
      "ready": 0.1,
      "generating": 0.2,
      "not checked": 0,
      "unavailable": 0,
      "unreachable application backend": 0,
      "unreachable": 0
    };

    function shouldAnimate(status: Status): boolean {
      return status === "ready" || status === "generating";
    }

    function updateShadow(): void {
      const color = statusColors[nex_status];
      const animate = shouldAnimate(nex_status);
      const rate = shadowQuantity[nex_status] || 0;

      if (animate) {
        shadowSize = growing ? shadowSize + rate : shadowSize - rate;
        growing = shadowSize >= 3 ? false : shadowSize <= rate ? true : growing;
      } else {
        shadowSize = 1.5;
      }

      if (logo) {
        logo.style.filter = `drop-shadow(0 0 ${shadowSize}em ${color})`;
      }
    }

    return setInterval(updateShadow, 100);
  }

  // Update the title text incrementally
  function updateTitle(): void {
    if (nex_title.length < title_letters.length) {
      nex_title += title_letters[nex_title.length];
    }
  }

  // Clear output
  function clearOutput(): void {
    raw_user_output = "";
    clearing = true;
    const clearIntervalId = setInterval(() => {
      removeOutput();
      if (user_output.length <= 1) {
        clearInterval(clearIntervalId);
        clearing = false;
      }
    }, 1);
  }

  // Append output incrementally
  function appendOutput(): void {
    if (!clearing && user_output.length < raw_user_output.length) {
      user_output += raw_user_output[user_output.length];
    }
  }

  // Remove one character each iteration from user_output
  function removeOutput(): void {
    user_output = user_output.length > 2 ? user_output.slice(0, -1) : "";
  }

  // Initialize on mount
  onMount(() => {
    scheduler();
    setInterval(updateTitle, 100);
    testBackend();
    checkAIStatus();
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