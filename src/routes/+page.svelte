<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import NewGame from "./NewGame.svelte";
  import type { Game } from "../lib";
  import ErrorPopup from "./ErrorPopup.svelte";
  import Chevron from "./Chevron.svelte";

  enum Dir {
    Up,
    Down,
  }

  let newGameMenu: any;

  let errorMessage = $state("");
  let games: Game[] = $state([]);
  let selectedGameIndex = $state(0);
  let selectedGame: Game = $derived(games[selectedGameIndex]);

  onMount(() => {
    refresh();
  });

  function refresh() {
    invoke<Game[]>("current_games")
      .then((newGames: Game[]) => (games = newGames))
      .catch((err: string) => (errorMessage = err));
  }

  function onGameSelected(index: number) {
    selectedGameIndex = index;
  }

  function editGame(game: Game) {
    newGameMenu.startEditGame(game);
  }

  function launchGame(game: Game) {
    invoke("launch_game", { command: game.launch_command }).catch(
      (err: string) => (errorMessage = err),
    );
  }

  function moveGame(dir: Dir, index: number) {
    if (
      (dir == Dir.Up && index == 0) ||
      (dir == Dir.Down && index == games.length - 1)
    ) {
      return;
    }

    invoke("move_game", {
      game_index: index,
      new_index: index + (dir == Dir.Down ? 1 : -1),
    })
      .then(() => refresh())
      .catch((err: string) => (errorMessage = err));
  }
</script>

<div class="flex flex-row p-4 gap-4 w-screen h-screen">
  <div class="flex flex-col gap-4 overflow-y-auto w-40">
    {#each games as game, index}
      <div style="background-color: {game.bg_color}; color: {game.text_color}">
        <button
          onclick={() => onGameSelected(index)}
          class="min-w-32 w-full min-h-40 flex items-center cursor-pointer p-4"
        >
          <p class="w-full font-bold text-center text-xl">
            {game.name}
          </p>
        </button>
        <hr class="mx-4" />
        <div class="flex flex-row justify-evenly">
          <button
            class="cursor-pointer"
            onclick={() => moveGame(Dir.Up, index)}
          >
            <Chevron class="fill-current rotate-90 size-8" />
          </button>
          <button
            class="cursor-pointer"
            onclick={() => moveGame(Dir.Down, index)}
          >
            <Chevron class="fill-current -rotate-90 size-8" />
          </button>
        </div>
      </div>
    {/each}
    <NewGame
      onChange={refresh}
      onError={(err: string) => (errorMessage = err)}
      bind:this={newGameMenu}
    />
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

<ErrorPopup message={errorMessage} />
