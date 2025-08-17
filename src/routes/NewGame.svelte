<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { Game } from "../lib";

  let menuOpen = $state(false);
  let newName = $state("");
  let newLaunch = $state("");

  async function addGame(event: Event) {
    event.preventDefault();
    const newGame: Game = {
      name: newName,
      launch_command: newLaunch,
    };
    await invoke("add_game", { game: newGame });
    menuOpen = false;
  }
</script>

<button onclick={() => (menuOpen = true)}>New Game</button>
{#if menuOpen}
  <div class="fixed top-0 left-0 w-full h-full p-8 backdrop-blur-md">
    <div class="p-4 h-full bg-slate-400 flex flex-col gap-4">
      <input
        placeholder="Name"
        class="w-full p-4 bg-slate-500"
        bind:value={newName}
      />
      <input
        placeholder="Launch Command"
        class="w-full p-4 bg-slate-500 font-mono"
        bind:value={newLaunch}
      />
      <input
        type="submit"
        class="p-4 w-full bg-slate-500 cursor-pointer"
        value="Add New Game"
        onclick={addGame}
      />
    </div>
  </div>
{/if}
