<script lang="ts">
  import { writeFile, FsTextFileOption } from "@tauri-apps/api/fs";
  import { dialog, invoke } from "@tauri-apps/api";
  import Measure from "./usables/Measure.svelte";
  let userInput: string = "";

  const writeToFile = async () => {
    const whereToSave: string = await dialog.save({
      filters: [{ name: "Text File", extensions: ["txt"] }],
    });
    if (!whereToSave) {
      await dialog.message("You have cancelled saving your file", {
        title: "Error",
        type: "error",
      });
      return;
    }
    const f: FsTextFileOption = {
      path: whereToSave,
      contents: userInput,
    };
    await writeFile(f);
  };

  const showOpenDialog = async () => {
    const result = await dialog.open({
      title: "Open File",
      multiple: false,
      recursive: true,
    });
    const contents: string = await invoke("parse_file", { path: result });
    userInput = contents;
  };
</script>

<main>
  <h1>Measure performance and manage files</h1>
  <button on:click={writeToFile}>Save file</button>
  <br />
  <button on:click={showOpenDialog}>Open file</button>
  <br />
  <input type="text" bind:value={userInput} placeholder="Your input..." />
  <hr />
  <p>{userInput}</p>
  <Measure />
</main>

<style>
  button {
    width: 100%;
  }

  input[type="text"] {
    width: 100%;
  }
</style>
