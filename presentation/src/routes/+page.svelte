<script lang="ts">
  import { goto } from "$app/navigation";
  import Button from "./button.svelte";

</script>

<div class="flex items-center justify-center flex-col h-full w-full">
  <div
    class="h-full flex flex-center justify-center flex-col w-full max-w-[50rem]"
  >
    <div class="grid grid-cols-1 gap-2 mb-8">
      <Button on:click={()=> goto("/questioner")}>START</Button>
    </div>
  </div>
</div>

<style>
</style>
