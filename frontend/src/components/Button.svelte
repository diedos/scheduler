<script lang="ts">
    export let color: 'blue' | 'green' | 'neutral' = 'neutral';
    export let square: boolean = false;
    export let disabled = false;
    export let type: 'button' | 'submit' = 'submit';
</script>

<button
    on:click|stopPropagation
    {type}
    class="transition-all h-auto w-auto rounded-md p-1 bg-emerald-600 text-gray-50 flex flex-row items-center space-x-1"
    class:bg-sky-500={color === 'blue'}
    class:hover:bg-sky-400={color === 'blue'}
    class:bg-emerald-500={color === 'green'}
    class:hover:bg-emerald-400={color === 'green'}
    class:bg-neutral-200={color === 'neutral'}
    class:hover:bg-neutral-100={color === 'neutral'}
    class:text-gray-800={color === 'neutral'}
    class:aspect-square={square}
    class:px-4={!square}
    class:bg-gray-100={disabled}
    class:hover:bg-gray-100={disabled}
    class:text-gray-400={disabled}
    class:italic={disabled}
    {disabled}
>
    <slot />
</button>
