<script lang="ts">
    export let color: 'blue' | 'green' | 'neutral' = 'neutral';
    export let square: boolean = false;
    export let disabled: boolean = false;
    export let type: 'button' | 'submit' = 'submit';
    export let large: boolean = false;
</script>

<button
    on:click|stopPropagation
    {type}
    class="transition-all h-auto w-auto rounded-md p-1 bg-emerald-60 flex flex-row items-center space-x-1"
    class:bg-sky-500={!disabled && color === 'blue'}
    class:hover:bg-sky-400={!disabled && color === 'blue'}
    class:bg-emerald-500={!disabled && color === 'green'}
    class:hover:bg-emerald-400={!disabled && color === 'green'}
    class:bg-neutral-200={!disabled && color === 'neutral'}
    class:hover:bg-neutral-100={!disabled && color === 'neutral'}
    class:text-gray-800={!disabled && color === 'neutral'}
    class:aspect-square={square}
    class:px-4={!square}
    class:bg-gray-100={disabled}
    class:hover:bg-gray-100={disabled}
    class:text-gray-50={!disabled}
    class:text-gray-400={disabled}
    class:italic={disabled}
    class:py-4={large}
    {disabled}
>
    <slot />
</button>
