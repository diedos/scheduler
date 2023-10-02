<script lang="ts">
    import { enhance } from '$app/forms';
    import { slide } from 'svelte/transition';
    import Box from './Box.svelte';

    export let show: boolean = false;
</script>

{#if show}
    <Box>
        <form action="?/create" method="post" use:enhance class="flex flex-col space-y-4 w-full">
            <div class="flex flex-row w-full space-x-4">
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
            </div>
            <div class="flex flex-row w-full space-x-4">
                <input
                    type="datetime-local"
                    name="deadlineAt"
                    class="p-2 border rounded-md w-1/4"
                />
                <input
                    type="range"
                    name="estimate"
                    min="1"
                    max="255"
                    class="p-2 border rounded-md flex-1"
                />
                <button
                    class="bg-emerald-600 text-gray-50 px-4 rounded-md border w-1/6"
                    type="submit">Create task</button
                >
            </div>
        </form>
    </Box>
{/if}
