<script lang="ts">
    import EditPost from "../../lib/editPost.svelte";
    import type { Delta } from "quill/core";
    import { getPost } from "../../lib/request";

    let postTitle: string = "";
    let postContents: Delta | null = null;

    const path = window.location.pathname.split("/");

    getPost(parseInt(path[path.length - 1]))
        .then((res) => {
            postTitle = res.title;
            postContents = res.content;
        })
        .catch(console.error);
</script>

<div class="w-full h-full py-7 px-3 mx-auto max-w-[calc(940px_+_15vw)]">
    {#if postContents == null}
        <h3 class="h3">Loading Post...</h3>
    {:else}
        <EditPost bind:postTitle bind:postContents />
    {/if}
</div>
