<script lang="ts">
    import { enhance } from '$app/forms';
    import { slide } from 'svelte/transition';

    export let show: boolean = false;
</script>

{#if show}
    <section
        class="bg-gray-50 text-gray-800 dark:bg-slate-800 flex flex-col rounded-xl dark:text-slate-300 overflow-hidden border p-4 w-full"
        transition:slide
    >
        <div>
            <form action="?/create" method="post" use:enhance class="flex flex-row space-x-4">
                <input
                    class="p-2 border rounded-md w-1/4"
                    placeholder="Task title"
                    type="text"
                    name="title"
                />
                <input
                    class="p-2 border rounded-md flex-1"
                    placeholder="Task description"
                    type="text"
                    name="content"
                />
                <button class="bg-emerald-600 text-gray-50 px-4 rounded-md border" type="submit"
                    >Create task</button
                >
            </form>
        </div>
    </section>
{/if}
