<script lang="ts">
    import { enhance } from '$app/forms';
    import { currentTask } from '../store';
    import Box from './Box.svelte';
    import Button from './Button.svelte';
    import SignInWithGoogle from './auth/SignInWithGoogle.svelte';

    export let show: boolean = false;
</script>

{#if show}
    <Box>
        <form
            action="?/create"
            method="post"
            class="flex flex-col space-y-4 w-full"
            use:enhance={({ formElement, formData, action, cancel, submitter }) => {
                return async ({ result, update }) => {
                    if (!$currentTask?.id) currentTask.set(result.data.createdTask);
                    update();
                };
            }}
        >
            <div class="flex lg:flex-row max-lg:flex-col w-full lg:space-x-4 max-lg:space-y-4">
                <input
                    class="p-2 border rounded-md lg:w-1/3"
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
            <div class="flex lg:flex-row max-lg:flex-col w-full lg:space-x-4 max-lg:space-y-4">
                <div class="flex flex-row max-lg:space-y-2 space-x-2 items-center lg:w-1/3">
                    <span class="flex">Deadline&nbsp;at</span>
                    <input
                        type="datetime-local"
                        name="deadlineAt"
                        class="p-2 border rounded-md flex-grow"
                    />
                </div>
                <div
                    class="flex lg:flex-row max-lg:flex-col max-lg:space-y-2 lg:space-x-2 lg:items-center lg:w-1/3"
                >
                    <div class="flex flex-row space-x-2 items-center">
                        <span class="flex">Time&nbsp;to&nbsp;complete</span>
                        <select name="estimate" class="p-2 border rounded-md">
                            <option value="0">Unestimated</option>
                            <option value="1">1 hour</option>
                            <option value="2">2 hours</option>
                            <option value="4">4 hours</option>
                            <option value="24">1 day</option>
                        </select>
                    </div>
                </div>

                <div class="flex flex-row lg:justify-end max-lg:justify-center flex-1">
                    <Button color="blue" large>Create task</Button>
                </div>
            </div>
        </form>
        <SignInWithGoogle />
    </Box>
{/if}
