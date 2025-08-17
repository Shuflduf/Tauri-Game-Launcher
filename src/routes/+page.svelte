<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import NewGame from "./NewGame.svelte";
  import type { Game } from "../lib";
  import ErrorPopup from "./ErrorPopup.svelte";

  let newGameMenu: any;

  let errorMessage = $state("");
  let games: Game[] = $state([]);
  let selectedGameIndex = $state(0);
  let selectedGame: Game = $derived(games[selectedGameIndex]);

  onMount(async () => {
    await refresh();
  });

  async function refresh() {
    invoke<Game[]>("current_games")
      .then((newGames: Game[]) => {
        games = newGames;
      })
      .catch((err: string) => {
        errorMessage = err;
      });
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

<ErrorPopup message={errorMessage} />

<div class="flex flex-row p-4 gap-4 w-screen h-screen">
  <div class="flex flex-col gap-4 overflow-y-auto w-40">
    {#each games as game, index}
      <button
        onclick={() => onGameSelected(index)}
        class="w-32 min-h-40 flex items-center p-4 cursor-pointer"
        style="background-color: {game.bg_color}; color: {game.text_color}"
      >
        <p class="w-full font-bold text-center">
          {game.name}
        </p>
      </button>
    {/each}
    <NewGame onChange={refresh} bind:this={newGameMenu} />
  </div>
  <div
    class="h-full w-full p-4 flex justify-between flex-col"
    style="background-color:
    {selectedGame ? selectedGame.bg_color : '#cad5e2'};"
  >
    {#if games[selectedGameIndex]}
      <div style="color: {selectedGame.text_color};">
        <h1 class="text-center text-4xl font-bold">{selectedGame.name}</h1>
        <hr class="my-4" />
        <p class="text-center">{selectedGame.description}</p>
      </div>
      <div class="w-full flex flex-row gap-4">
        <button
          class="bg-green-400 p-4 w-full cursor-pointer"
          onclick={() => launchGame(selectedGame)}
          title={selectedGame.launch_command}
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
