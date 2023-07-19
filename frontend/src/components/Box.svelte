<script lang="ts">
    /** Optional inline styling */
    export let style: string = '';
</script>

<div
    class="bg-gray-50 text-gray-800 dark:bg-slate-800 flex rounded-xl dark:text-slate-300 overflow-hidden border border-gray-300 dark:border-gray-700 p-4 {style}"
>
    <slot />
</div>
