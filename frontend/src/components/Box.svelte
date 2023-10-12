<script lang="ts">
    export let itemAlign: 'start' | 'center' | 'end' = 'start';
    export let hoverable: boolean = false;
</script>

<div
    class="group bg-gradient-to-tr from-sky-500 to-emerald-500 text-gray-800 flex flex-row w-full overflow-hidden items-center p-1 rounded-lg"
>
    <div
        class="bg-gray-50 text-gray-800 flex flex-row w-full overflow-hidden px-6 py-6 rounded-md transition-all"
        class:items-start={itemAlign === 'start'}
        class:items-center={itemAlign === 'center'}
        class:items-end={itemAlign === 'end'}
        class:group-hover:bg-white={hoverable}
    >
        <slot />
    </div>
</div>
