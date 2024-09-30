<script lang="ts">
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    export let disabled: boolean = false;
</script>

<button
    class={[
        ...[
            "text-xl border-b-2 rounded w-full border-gray-200 p-3 text-center text-gray-700",
            disabled == false ? "bg-white active:bg-gray-50" : "",
            disabled ? " bg-gray-200" : "",
        ],
    ].join(" ")}
    {disabled}
    tabindex="-1"
    on:click={() => dispatch("click")}
>
    <slot />
</button>
