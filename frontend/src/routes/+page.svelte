<script lang="ts">
    import { onMount } from 'svelte';
    import Task from '../components/Task.svelte';
    import { todoTasks } from '../store';

    onMount(async () => {
        fetch('http://localhost:3000/tasks')
            .then((response) => response.json())
            .then((data) => {
                console.log(data);
                todoTasks.set(data);
            })
            .catch((error) => {
                console.log(error);
                return [];
            });
    });
</script>

<div class="flex flex-row">
    {#each $todoTasks as task}
        <Task
            title={task.title}
            content={task.content}
            createdAt={task.createdAt}
            completed={task.completed}
        />
    {/each}
</div>
