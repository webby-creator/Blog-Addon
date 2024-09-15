<script lang="ts">
    import EditPost from "../../lib/editPost.svelte";
    import type { Delta } from "quill/core";

    let postTitle: string = "";
    let postContents: Delta | null = null;
</script>

<div class="w-full h-full py-7 px-3 mx-auto max-w-[calc(940px_+_15vw)]">
    <EditPost bind:postTitle bind:postContents />
</div>
