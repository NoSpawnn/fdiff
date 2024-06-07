<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let text_left = "";
  let text_right = "";
  let response = "";
  let submitted = false;

  function setInputState() {
    let text_inputs = document.getElementsByClassName("text-input");
    let text_submitted = document.getElementsByClassName("text-submitted");

    for (let i = 0; i < text_inputs.length; i++) {
      let input_elem = text_inputs.item(i) as HTMLTextAreaElement;
      input_elem.style.display = submitted ? "none" : "block";

      let submit_elem = text_submitted.item(i) as HTMLDivElement;
      submit_elem.style.display = submitted ? "block" : "none";
    }
  }

  async function submit(event: Event) {
    submitted = true;
    setInputState();

    invoke("submit", { text1: text_left, text2: text_right }).then((v) => {
      response = v as string;
    });
  }

  async function unsubmit() {
    submitted = false;
    setInputState();
  }
</script>

<div class="container">
  <div class="text-container">
    <div class="text-input-container">
      <textarea class="text-input" bind:value={text_left} />
      <textarea class="text-input" bind:value={text_right} />
    </div>
    <div class="text-submitted-container">
      <div class="text-submitted">{text_left}</div>
      <div class="text-submitted">{text_right}</div>
    </div>
  </div>

  {#if submitted}
    <button class="text-submit" type="submit" on:click={unsubmit}>Edit</button>
  {:else}
    <button class="text-submit" type="submit" on:click={submit}>Diff</button>
  {/if}
</div>

<style>
  .container {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    gap: 1rem;
    margin: 0.5rem;
    width: 100%;
  }

  .text-container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    width: 100%;
  }

  .text-input-container,
  .text-submitted-container {
    display: flex;
    justify-content: center;
    gap: 1rem;
    height: inherit;
  }

  .text-input {
    resize: none;
    border: solid 2px var(--active-outline-color);
    border-radius: 5px;
    width: 45vw;
    font-size: 15px;
    padding: 0.3rem;
    color: var(--text-color);
    background-color: var(--secondary-bg-color);
  }

  .text-input:focus {
    border: solid 2px var(--active-outline-color);
    outline: none;
  }

  .text-submitted {
    display: none;
    border: solid 2px var(--inactive-outline-color);
    border-radius: 5px;
    padding: 0.3rem;
    font-size: 15px;
    width: 45vw;
    color: var(--text-color);
    background-color: var(--secondary-bg-color);
  }

  button {
    border: none;
    border-radius: 5px;
    padding: 0.4rem;
    width: 10rem;
    background-color: var(--secondary-bg-color);
    color: var(--text-color);
    border: solid 2px var(--active-outline-color);
  }
</style>
