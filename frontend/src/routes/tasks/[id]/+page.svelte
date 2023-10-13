<script>
    import { enhance } from '$app/forms';
    import Box from 'components/Box.svelte';
    import NavButton from 'components/NavButton.svelte';

    export let data;

    console.log(data);
</script>

Nothing to see here yet :(

<!-- <Box>
    <div class="flex flex-col flex-1 space-y-4">
        <section class="flex flex-col space-y-4">
            <h2 class="text-2xl">{data.title}</h2>
            <span>{data.content}</span>
        </section>
    </div>
    <aside class="flex flex-col w-48 space-y-2">
        <form method="POST" action="/?/complete" use:enhance>
            <input type="hidden" name="id" value={data.id} />
            <button
                type="submit"
                class="flex w-full px-4 py-2 text-left bg-emerald-500 rounded-md text-gray-50 font-bold"
                >Mark as done</button
            >
        </form>
        <form method="POST" action="/?/delete" use:enhance>
            <input type="hidden" name="id" value={data.id} />
            <button
                type="submit"
                class="flex w-full px-4 py-2 text-left bg-rose-700 rounded-md text-gray-50"
                >Delete</button
            >
        </form>
        <span class="text-sm">Created at<br />{data.createdAt}</span>
        <span class="text-sm">Updated at<br />{data.updatedAt}</span>
        <span class="text-sm">Completed at<br />{data.completedAt}</span>
        <span class="text-sm">Deadline at<br />{data.deadlineAt}</span>
        <span class="text-sm"
            >Estimate<br />{data.estimate} {data.estimate === 1 ? 'hour' : 'hours'}</span
        >
    </aside>
</Box> -->
