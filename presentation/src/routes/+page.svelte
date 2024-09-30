<script lang="ts">
  import { goto } from "$app/navigation";
  import Button from "../button.svelte";
  import Fire from "../icons/fire.svelte";
  import Star from "../icons/star.svelte";

  import type { PageData } from './$types';
	
	export let data: PageData;
</script>

<div class="flex items-center justify-center flex-col h-full w-full">
  <div
    class="h-full flex justify-between items-center flex-col w-full"
  >
    <div class="text-gray-400 flex justify-between w-full">
      <div class="flex gap-1 items-center">
        <Star></Star>
        <p>{data.stats.high_score}</p>
      </div>

      <div class="flex gap-1 items-center">
        <Fire></Fire>
        <p>{data.stats.daily_streak}</p>
      </div>
    </div>

    <div class="grid grid-cols-1 gap-2 mb-8 max-w-[20rem] w-full">
      <h1 class="text-lg text-center text-gray-400">
        Previous Score: {data.stats.previous_score}
      </h1>
      <Button on:click={() => goto("/questioner")}>START</Button>
    </div>

    <div></div>
  </div>
</div>

<style>
</style>
