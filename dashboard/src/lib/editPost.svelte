<script lang="ts">
    import { onMount } from "svelte";

    import Quill from "quill";
    import type { Delta } from "quill/core";

    import { createPost, updatePost } from "./request";

    export let postId: number | null = null;
    export let postTitle = "";
    export let postContents: Delta | null = null;

    let quill: Quill;

    let postLength = 0;

    onMount(() => {
        quill = new Quill("#editor", {
            theme: "snow",
            modules: {
                toolbar: "#editor-toolbar",
            },
        });

        if (postContents != null) {
            quill.setContents(postContents);
        }

        let timeout: number | null = null;

        quill.on("text-change", () => {
            postLength = quill.getLength();

            if (timeout != null) {
                clearTimeout(timeout);
            }

            timeout = setTimeout(() => {
                timeout = null;

                // TODO: Save the content to the database as Draft
            }, 3000);
        });
    });
</script>

<link
    href="https://cdn.jsdelivr.net/npm/quill@2.0.2/dist/quill.snow.css"
    rel="stylesheet"
/>

<!-- Sidebar -->
<!-- Add: (Media) Image, AI Image, Gallery, Video, Gif, File, (Elements) Divider, Button, Table, Expandable List, Poll, (From Web) HTML Code, SoundCloud, AdSense, (other) Products, Services,  -->
<!-- Settings: (general) Featured Image, Publish Date, Writer, Excerpt, Related Posts, Feature Post?, Comments?, (categories), (tags) -->
<!-- SEO: -->
<!-- Monetize: -->
<!-- Translate: -->
<!-- AI Tools: Create Post, Create Title, Create an Outline, Create Images, Add Meta Tags -->

<div class="flex space-x-3">
    <div
        class="h-[calc(100%_-3.5rem)] rounded-lg w-[15vw] max-w-64 bg-slate-400 p-2 min-w-52"
    >
        <div class="flex">
            <button
                class="btn variant-filled rounded"
                disabled={postTitle.trim().length === 0 || postLength === 0}
                on:click={async () => {
                    if (postId == null) {
                        const resp = await createPost(
                            postTitle,
                            quill.getContents(),
                        );

                        console.log(resp);
                    } else {
                        const resp = await updatePost(postId, {
                            title: postTitle,
                            content: quill.getContents(),
                        });

                        console.log(resp);
                    }
                }}
            >
                {#if postId == null}
                    Create
                {:else}
                    Save
                {/if}
            </button>
        </div>
    </div>

    <!-- Toolbar -->
    <!-- Text Style (P, H1, H2), Text Size, Bold, Italicize, Underline, Text Color, Highlight Color, Quote, Code Snippet, Numbered List, Bulleted List, Alignment, Line Space, Dec/Inc Indent, Link,  -->
    <div class="space-y-3 h-full grow">
        <div>
            <!-- Title -->
            <input
                class="w-full h-14 text-xl rounded"
                placeholder="Add a Catchy Title"
                type="text"
                bind:value={postTitle}
            />
        </div>

        <div
            class="flex flex-col p-2 border border-slate-500 rounded bg-slate-50 min-h-[50%]"
        >
            <div
                id="editor-toolbar"
                class="toolbar ql-toolbar !border-none rounded bg-slate-200"
            >
                <!-- Font, Size -->
                <span class="ql-formats">
                    <select class="ql-font">
                        <option selected={true}></option>
                        <option value="serif"></option>
                        <option value="monospace"></option>
                    </select>
                    <select class="ql-size">
                        <option value="small"></option>
                        <option selected={true}></option>
                        <option value="large"></option>
                        <option value="huge"></option>
                    </select>
                </span>

                <!-- Bold, Italicize, Underline, Slash -->
                <span class="ql-formats">
                    <button class="ql-bold" type="button">
                        <svg viewBox="0 0 18 18">
                            <path
                                class="ql-stroke"
                                d="M5,4H9.5A2.5,2.5,0,0,1,12,6.5v0A2.5,2.5,0,0,1,9.5,9H5A0,0,0,0,1,5,9V4A0,0,0,0,1,5,4Z"
                            >
                            </path>
                            <path
                                class="ql-stroke"
                                d="M5,9h5.5A2.5,2.5,0,0,1,13,11.5v0A2.5,2.5,0,0,1,10.5,14H5a0,0,0,0,1,0,0V9A0,0,0,0,1,5,9Z"
                            >
                            </path>
                        </svg>
                    </button>
                    <button class="ql-italic" type="button">
                        <svg viewBox="0 0 18 18">
                            <line
                                class="ql-stroke"
                                x1="7"
                                x2="13"
                                y1="4"
                                y2="4"
                            >
                            </line>
                            <line
                                class="ql-stroke"
                                x1="5"
                                x2="11"
                                y1="14"
                                y2="14"
                            >
                            </line>
                            <line
                                class="ql-stroke"
                                x1="8"
                                x2="10"
                                y1="14"
                                y2="4"
                            >
                            </line>
                        </svg>
                    </button>
                    <button class="ql-underline" type="button">
                        <svg viewBox="0 0 18 18">
                            <path
                                class="ql-stroke"
                                d="M5,3V9a4.012,4.012,0,0,0,4,4H9a4.012,4.012,0,0,0,4-4V3"
                            >
                            </path>
                            <rect
                                class="ql-fill"
                                height="1"
                                rx="0.5"
                                ry="0.5"
                                width="12"
                                x="3"
                                y="15"
                            >
                            </rect>
                        </svg>
                    </button>
                    <button class="ql-strike" type="button">
                        <svg viewBox="0 0 18 18">
                            <line
                                class="ql-stroke ql-thin"
                                x1="15.5"
                                x2="2.5"
                                y1="8.5"
                                y2="9.5"
                            >
                            </line>
                            <path
                                class="ql-fill"
                                d="M9.007,8C6.542,7.791,6,7.519,6,6.5,6,5.792,7.283,5,9,5c1.571,0,2.765.679,2.969,1.309a1,1,0,0,0,1.9-.617C13.356,4.106,11.354,3,9,3,6.2,3,4,4.538,4,6.5a3.2,3.2,0,0,0,.5,1.843Z"
                            >
                            </path>
                            <path
                                class="ql-fill"
                                d="M8.984,10C11.457,10.208,12,10.479,12,11.5c0,0.708-1.283,1.5-3,1.5-1.571,0-2.765-.679-2.969-1.309a1,1,0,1,0-1.9.617C4.644,13.894,6.646,15,9,15c2.8,0,5-1.538,5-3.5a3.2,3.2,0,0,0-.5-1.843Z"
                            >
                            </path>
                        </svg>
                    </button>
                </span>

                <!-- Color -->
                <span class="ql-formats">
                    <select class="ql-color">
                        <option selected={true}></option>
                        <option value="#e60000"></option>
                        <option value="#ff9900"></option>
                        <option value="#ffff00"></option>
                        <option value="#008a00"></option>
                        <option value="#0066cc"></option>
                        <option value="#9933ff"></option>
                        <option value="#ffffff"></option>
                        <option value="#facccc"></option>
                        <option value="#ffebcc"></option>
                        <option value="#ffffcc"></option>
                        <option value="#cce8cc"></option>
                        <option value="#cce0f5"></option>
                        <option value="#ebd6ff"></option>
                        <option value="#bbbbbb"></option>
                        <option value="#f06666"></option>
                        <option value="#ffc266"></option>
                        <option value="#ffff66"></option>
                        <option value="#66b966"></option>
                        <option value="#66a3e0"></option>
                        <option value="#c285ff"></option>
                        <option value="#888888"></option>
                        <option value="#a10000"></option>
                        <option value="#b26b00"></option>
                        <option value="#b2b200"></option>
                        <option value="#006100"></option>
                        <option value="#0047b2"></option>
                        <option value="#6b24b2"></option>
                        <option value="#444444"></option>
                        <option value="#5c0000"></option>
                        <option value="#663d00"></option>
                        <option value="#666600"></option>
                        <option value="#003700"></option>
                        <option value="#002966"></option>
                        <option value="#3d1466"></option>
                    </select>
                    <select class="ql-background">
                        <option value="#000000"></option>
                        <option value="#e60000"></option>
                        <option value="#ff9900"></option>
                        <option value="#ffff00"></option>
                        <option value="#008a00"></option>
                        <option value="#0066cc"></option>
                        <option value="#9933ff"></option>
                        <option selected={true}></option>
                        <option value="#facccc"></option>
                        <option value="#ffebcc"></option>
                        <option value="#ffffcc"></option>
                        <option value="#cce8cc"></option>
                        <option value="#cce0f5"></option>
                        <option value="#ebd6ff"></option>
                        <option value="#bbbbbb"></option>
                        <option value="#f06666"></option>
                        <option value="#ffc266"></option>
                        <option value="#ffff66"></option>
                        <option value="#66b966"></option>
                        <option value="#66a3e0"></option>
                        <option value="#c285ff"></option>
                        <option value="#888888"></option>
                        <option value="#a10000"></option>
                        <option value="#b26b00"></option>
                        <option value="#b2b200"></option>
                        <option value="#006100"></option>
                        <option value="#0047b2"></option>
                        <option value="#6b24b2"></option>
                        <option value="#444444"></option>
                        <option value="#5c0000"></option>
                        <option value="#663d00"></option>
                        <option value="#666600"></option>
                        <option value="#003700"></option>
                        <option value="#002966"></option>
                        <option value="#3d1466"></option>
                    </select>
                </span>

                <!-- List Style & Text Position -->
                <span class="ql-formats">
                    <button class="ql-list" value="ordered" type="button">
                        <svg viewBox="0 0 18 18">
                            <line
                                class="ql-stroke"
                                x1="7"
                                x2="15"
                                y1="4"
                                y2="4"
                            >
                            </line>
                            <line
                                class="ql-stroke"
                                x1="7"
                                x2="15"
                                y1="9"
                                y2="9"
                            >
                            </line>
                            <line
                                class="ql-stroke"
                                x1="7"
                                x2="15"
                                y1="14"
                                y2="14"
                            >
                            </line>
                            <line
                                class="ql-stroke ql-thin"
                                x1="2.5"
                                x2="4.5"
                                y1="5.5"
                                y2="5.5"
                            >
                            </line>
                            <path
                                class="ql-fill"
                                d="M3.5,6A0.5,0.5,0,0,1,3,5.5V3.085l-0.276.138A0.5,0.5,0,0,1,2.053,3c-0.124-.247-0.023-0.324.224-0.447l1-.5A0.5,0.5,0,0,1,4,2.5v3A0.5,0.5,0,0,1,3.5,6Z"
                            >
                            </path>
                            <path
                                class="ql-stroke ql-thin"
                                d="M4.5,10.5h-2c0-.234,1.85-1.076,1.85-2.234A0.959,0.959,0,0,0,2.5,8.156"
                            >
                            </path>
                            <path
                                class="ql-stroke ql-thin"
                                d="M2.5,14.846a0.959,0.959,0,0,0,1.85-.109A0.7,0.7,0,0,0,3.75,14a0.688,0.688,0,0,0,.6-0.736,0.959,0.959,0,0,0-1.85-.109"
                            >
                            </path>
                        </svg>
                    </button>
                    <button class="ql-list" value="bullet" type="button">
                        <svg viewBox="0 0 18 18">
                            <line
                                class="ql-stroke"
                                x1="6"
                                x2="15"
                                y1="4"
                                y2="4"
                            >
                            </line>
                            <line
                                class="ql-stroke"
                                x1="6"
                                x2="15"
                                y1="9"
                                y2="9"
                            >
                            </line>
                            <line
                                class="ql-stroke"
                                x1="6"
                                x2="15"
                                y1="14"
                                y2="14"
                            >
                            </line>
                            <line class="ql-stroke" x1="3" x2="3" y1="4" y2="4">
                            </line>
                            <line class="ql-stroke" x1="3" x2="3" y1="9" y2="9">
                            </line>
                            <line
                                class="ql-stroke"
                                x1="3"
                                x2="3"
                                y1="14"
                                y2="14"
                            >
                            </line>
                        </svg>
                    </button>
                    <select class="ql-align">
                        <option selected={true}></option>
                        <option value="center"></option>
                        <option value="right"></option>
                        <option value="justify"></option>
                    </select>
                </span>

                <!-- Link, Image -->
                <span class="ql-formats">
                    <button class="ql-link" type="button">
                        <svg viewBox="0 0 18 18">
                            <line
                                class="ql-stroke"
                                x1="7"
                                x2="11"
                                y1="7"
                                y2="11"
                            >
                            </line>
                            <path
                                class="ql-even ql-stroke"
                                d="M8.9,4.577a3.476,3.476,0,0,1,.36,4.679A3.476,3.476,0,0,1,4.577,8.9C3.185,7.5,2.035,6.4,4.217,4.217S7.5,3.185,8.9,4.577Z"
                            >
                            </path>
                            <path
                                class="ql-even ql-stroke"
                                d="M13.423,9.1a3.476,3.476,0,0,0-4.679-.36,3.476,3.476,0,0,0,.36,4.679c1.392,1.392,2.5,2.542,4.679.36S14.815,10.5,13.423,9.1Z"
                            >
                            </path>
                        </svg>
                    </button>
                    <button class="ql-image" type="button">
                        <svg viewBox="0 0 18 18">
                            <rect
                                class="ql-stroke"
                                height="10"
                                width="12"
                                x="3"
                                y="4"
                            >
                            </rect>
                            <circle class="ql-fill" cx="6" cy="7" r="1">
                            </circle>
                            <polyline
                                class="ql-even ql-fill"
                                points="5 12 5 11 7 9 8 10 11 7 13 9 13 12 5 12"
                            >
                            </polyline>
                        </svg>
                    </button>
                </span>
            </div>

            <!-- TODO: For some reason the inner div h-full isn't working w/o grid -->
            <div id="editor" class="!border-none grow grid"></div>
        </div>
    </div>
</div>

<style>
    :global(#editor img) {
        display: unset;
    }
</style>
