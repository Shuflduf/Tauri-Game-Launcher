<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import NewGame from "./NewGame.svelte";
  import type { Game } from "../lib";

  let newGameMenu: any;

  let games: Game[] = $state([]);
  let selectedGameIndex = $state(0);

  onMount(async () => {
    // games = await invoke("current_games");
    await refresh();
  });

  async function refresh() {
    games = await invoke("current_games");
  }

  function onGameSelected(index: number) {
    selectedGameIndex = index;
  }

  function editGame(game: Game) {
    newGameMenu.startEditGame(game);
  }

  async function launchGame(game: Game) {
    const res = await invoke("launch_game", { command: game.launch_command });
    console.log(res);
  }
</script>

<div class="flex flex-row p-4 gap-4 w-screen h-screen">
  <div class="flex flex-col gap-4 overflow-y-auto w-40">
    {#each games as game, index}
      <button
        onclick={() => onGameSelected(index)}
        class="bg-slate-300 w-32 min-h-40 flex items-center p-4 cursor-pointer"
      >
        <p class="text-center w-full">
          {game.name}
        </p>
      </button>
    {/each}
    <NewGame onChange={refresh} bind:this={newGameMenu} />
  </div>
  <div class="bg-slate-300 h-full w-full p-4 flex justify-between flex-col">
    {#if games[selectedGameIndex]}
      {@const selectedGame = games[selectedGameIndex]}
      <div>
        <h1 class="text-center text-4xl font-bold">{selectedGame.name}</h1>
        <hr class="my-4" />
        <p class="text-center">{selectedGame.description}</p>
      </div>
      <div class="w-full flex flex-row gap-4">
        <button
          class="bg-green-400 p-4 w-full cursor-pointer"
          onclick={() => launchGame(selectedGame)}
        >
          Start
        </button>
        <button
          class="bg-slate-400 p-4 cursor-pointer w-full"
          onclick={() => selectedGame && editGame(selectedGame)}
        >
          Edit
        </button>
      </div>
    {/if}
  </div>
</div>
