<script lang="ts">
    export let showModal: boolean;
    let dialog: HTMLDialogElement;

    $: if (dialog && showModal) dialog.showModal();
</script>

<dialog
    bind:this={dialog}
    on:close={() => (showModal = false)}
    on:click|self={() => dialog.close()}
>
    <slot />
</dialog>
