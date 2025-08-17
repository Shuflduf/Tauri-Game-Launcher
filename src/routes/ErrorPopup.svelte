<script lang="ts">
  let menuOpen = $state(false);

  let { message }: { message: string } = $props();
  $effect(() => {
    if (message) {
      menuOpen = true;
    }
  });
</script>

{#if menuOpen}
  <div
    class="fixed top-0 left-0 w-full h-full p-12 backdrop-blur-md text-white"
  >
    <div class="p-4 h-full bg-slate-500 flex flex-col gap-4">
      {message}
    </div>
  </div>
{/if}
