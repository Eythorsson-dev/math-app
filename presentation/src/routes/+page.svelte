<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { createCountdown } from "../stores/contdown";
  import type { Readable } from "svelte/store";

  function getEquation(): Promise<{ expression: string[]; answer: number }> {
    return invoke("get_equation");
  }

  function getSettings(): Promise<{
    correct_audio_src: string | undefined;
    game_duration_sec: number;
  }> {
    return invoke("get_settings");
  }

  function setNextEquation() {
    getEquation().then((value) => {
      equation = value.expression;
      answer = value.answer;
      answerOptions = getAnswerOptions();
    });
  }

  const numberOfOptions = 6;

  function getAnswerOptions() {
    const minOption =
      answer - Math.round(Math.random() * (numberOfOptions - 1));
    return new Array(numberOfOptions).fill(0).map((_, i) => minOption + i);
  }

  let audio: HTMLAudioElement;
  let audioUrl: string | undefined;

  let countdown: Readable<String>;
  getSettings().then((x) => {
    audioUrl = x.correct_audio_src;

    countdown = createCountdown(x.game_duration_sec, () => {
      console.log("DONE!!");
    });

    setNextEquation();
  });

  let answer: number = 0;
  let equation: string[] = [];
  let answerOptions: number[] = [];

  let answered: number[] = [];

  function chooseAnswer(value: number) {
    if (value != answer) {
      answered = [...answered, value];
      return;
    }

    setNextEquation();

    audio.currentTime = 0;
    audio.play();
    answered = [];
  }

  function parse(items: string[]): { value: string; colorIndex: number }[] {
    function getIndexIncrement(
      previous: string | undefined,
      current: string,
    ): -1 | 0 | 1 {
      if (current == "(") return 1;
      else if (previous == ")") return -1;

      return 0;
    }

    let colorIndex = 0;
    return items.map((x, i) => ({
      value: x,
      colorIndex: (colorIndex += getIndexIncrement(items[i - 1], x)),
    }));
  }

  function getColorClass(colorIndex: number): string {
    var colorClasses = ["text-slate-600", "text-orange-400"];

    return colorClasses[colorIndex % colorClasses.length];
  }
</script>

<div class="flex items-center justify-center flex-col h-full w-full">
  <div class="h-full flex flex-center flex-col justify-between w-full">
    <h1 class="text-md text-center text-gray-400">{$countdown}</h1>

    <div class="text-center w-full">
      {#each parse(equation) as { value, colorIndex }}
        <span class="p-0.5 text-3xl {getColorClass(colorIndex)}">{value}</span>
      {/each}
    </div>

    <div class="grid grid-cols-2 mb-8">
      {#each answerOptions as option, index}
        <button
          class={[
            ...[
              "text-xl border-b-2 rounded w-full border-gray-200 p-3 m-1 text-center text-gray-700",
              answered.includes(option) == false
                ? "bg-white active:bg-gray-50"
                : "",
              answered.includes(option) ? " bg-gray-200" : "",
            ],
          ].join(" ")}
          class:justify-self-end={index % 2 == 0}
          disabled={answered.includes(option)}
          on:click={() => chooseAnswer(option)}
          tabindex="-1">{option}</button
        >
      {/each}
    </div>
  </div>
  <audio src={audioUrl} bind:this={audio} preload="auto"></audio>
</div>

<style>
</style>
