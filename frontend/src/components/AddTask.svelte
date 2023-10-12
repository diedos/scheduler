<script lang="ts">
    import { enhance } from '$app/forms';
    import Box from './Box.svelte';
    import Button from './Button.svelte';

    export let show: boolean = false;
</script>

{#if show}
    <Box>
        <form action="?/create" method="post" use:enhance class="flex flex-col space-y-4 w-full">
            <div class="flex flex-row w-full space-x-4">
                <input
                    class="p-2 border rounded-md w-1/3"
                    placeholder="Task title"
                    type="text"
                    name="title"
                    required
                />
                <input
                    class="p-2 border rounded-md flex-1"
                    placeholder="Task description"
                    type="text"
                    name="content"
                />
            </div>
            <div class="flex flex-row w-full space-x-4">
                <div class="flex flex-row space-x-2 items-center w-1/3">
                    <span class="flex">Deadline at</span>
                    <input
                        type="datetime-local"
                        name="deadlineAt"
                        class="p-2 border rounded-md flex-grow"
                    />
                </div>
                <div class="flex flex-row w-1/3">
                    <div class="flex flex-row space-x-2 items-center">
                        <span class="flex">Time to complete</span>
                        <select name="estimate" class="p-2 border rounded-md">
                            <option value="0">Unestimated</option>
                            <option value="1">1 hour</option>
                            <option value="2">2 hours</option>
                            <option value="4">4 hours</option>
                            <option value="24">1 day</option>
                        </select>
                    </div>
                </div>

                <div class="flex flex-row justify-end flex-1">
                    <Button color="blue">Create task</Button>
                </div>
            </div>
        </form>
    </Box>
{/if}
