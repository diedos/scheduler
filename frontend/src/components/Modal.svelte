<script lang="ts">
    import { enhance } from '$app/forms';

    export let showModal: boolean;
    let dialog: HTMLDialogElement;

    $: if (dialog && showModal) dialog.showModal();
</script>

<dialog
    bind:this={dialog}
    on:close={() => (showModal = false)}
    on:click|self={() => dialog.close()}
>
    <div>
        <form action="?/create" method="post" use:enhance>
            <input type="text" name="title" /><button type="submit">New task</button>
        </form>
    </div>
</dialog>
