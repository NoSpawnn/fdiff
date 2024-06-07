<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  interface LineDiff {
    left_text: string;
    right_text: string;
    is_diff: boolean;
  }

  let left_text_input = "";
  let right_text_input = "";
  let left_text_diff = "";
  let right_text_diff = "";
  let submitted = false;
  
  function scroll(e: Event) {
    let left_text_area = document.getElementById("left-text-input");
    let right_text_area = document.getElementById("right-text-input");
    let left_text_submitted = document.getElementById("left-text-submitted");
    let right_text_submitted = document.getElementById("right-text-submitted");

    switch (e.target!.id) {
      case "left-text-input":
        right_text_area!.scrollTop = left_text_area!.scrollTop;
        break;
      case "right-text-input":
        left_text_area!.scrollTop = right_text_area!.scrollTop;
        break;
      case "left-text-submitted":
        right_text_submitted!.scrollTop = left_text_submitted!.scrollTop;
        break;
      case "right-text-submitted":
        left_text_submitted!.scrollTop = right_text_submitted!.scrollTop;
        break;
    }
  }

  async function getDiff() {
    submitted = true;

    left_text_diff = "";
    right_text_diff = "";

    invoke("get_diff", {
      left_text: left_text_input,
      right_text: right_text_input,
    }).then((response) => {
      (response as LineDiff[]).forEach((diffEntry) => {
        if (diffEntry.is_diff) {
          left_text_diff += `<span style="background-color: var(--text-diff-bg-color); display: block;">${diffEntry.left_text}</span> ${diffEntry.left_text === "" ? "<br>" : ""}`;
          right_text_diff += `<span style="background-color: var(--text-diff-bg-color); display: block;">${diffEntry.right_text}</span> ${diffEntry.right_text === "" ? "<br>" : ""}`;
        } else {
          left_text_diff += diffEntry.left_text + "<br>";
          right_text_diff += diffEntry.right_text + "<br>";
        }
      });
    });
  }

  function editInput() {
    submitted = false;
  }

  function clearText() {
    left_text_input = "";
    right_text_input = "";
    submitted = false;
  }
</script>

<div class="container">
  <div class="text-container">
    {#if !submitted}
      <div class="text-input-container">
        <textarea
          class="text-input"
          id="left-text-input"
          on:scroll={scroll}
          bind:value={left_text_input}
        />
        <textarea
          class="text-input"
          id="right-text-input"
          on:scroll={scroll}
          bind:value={right_text_input}
        />
      </div>
    {:else}
      <div class="text-submitted-container">
        <div class="text-submitted" on:scroll={scroll} id="left-text-submitted">
          {@html left_text_diff}
        </div>
        <div
          class="text-submitted"
          on:scroll={scroll}
          id="right-text-submitted"
        >
          {@html right_text_diff}
        </div>
      </div>
    {/if}
  </div>

  <div class="buttons-container">
    {#if submitted}
      <button class="text-submit" type="submit" on:click={editInput}
        >Edit</button
      >
    {:else}
      <button class="text-submit" type="submit" on:click={getDiff}>Diff</button>
    {/if}
    <button class="text-submit" on:click={clearText}>Clear</button>
  </div>
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
    max-height: 100%;
    width: 100%;
    max-width: 100%;
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
    max-width: 45vw;
    font-size: 15px;
    padding: 0.3rem;
    background-color: var(--secondary-bg-color);
  }

  .text-input:focus {
    border: solid 2px var(--active-outline-color);
    outline: none;
  }

  .text-submitted {
    border: solid 2px var(--inactive-outline-color);
    border-radius: 5px;
    padding: 0.3rem;
    font-size: 15px;
    width: 45vw;
    max-width: 45vw;
    background-color: var(--secondary-bg-color);
    white-space: pre-wrap;
    overflow: auto;
  }

  .buttons-container {
    display: flex;
    flex-direction: row;
    gap: 1rem;
  }

  button {
    border: none;
    border-radius: 5px;
    padding: 0.4rem;
    width: 10rem;
    background-color: var(--secondary-bg-color);
    border: solid 2px var(--active-outline-color);
  }

  button:hover {
    cursor: pointer;
  }
</style>
