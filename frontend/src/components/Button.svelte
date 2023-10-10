<script lang="ts">
    export let color: 'blue' | 'green' = 'blue';
</script>

<button
    on:click|stopPropagation
    type="submit"
    class="  transition-all h-auto w-auto rounded-md p-1 bg-emerald-600 text-gray-50"
    class:bg-sky-500={color === 'blue'}
    class:hover:bg-sky-400={color === 'blue'}
    class:bg-emerald-500={color === 'green'}
    class:hover:bg-emerald-400={color === 'green'}
>
    <slot />
</button>
