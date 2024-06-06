<script async>
	import { onMount } from 'svelte';
	import { Card, Button, Toggle } from 'flowbite-svelte';
	import {
		Section,
		News,
		HeroHeader,
		HeroBody,
		Quotes,
		Testimonial,
		TestimonialCard,
		TestimonialCardItem
	} from 'flowbite-svelte-blocks';
	import { Pagination } from 'flowbite-svelte';
	import { ChevronLeftOutline, ChevronRightOutline } from 'flowbite-svelte-icons';
	import { ArrowRightOutline } from 'flowbite-svelte-icons';
	import { fade } from 'svelte/transition';
	import { sineInOut } from 'svelte/easing';
	import { page } from '$app/stores';
</script>

<div
	class="h-[calc(100vh-80px)] w-full relative bg-[url('/src/lib/images/blob_1_light.svg')] dark:bg-[url('/src/lib/images/blob_1_dark.svg')] bg-cover bg-center bg-no-repeat p-10"
>
	<div class=" h-full w-full flex flex-col items-center justify-center gap-[3vw]">
		<div class=" place-self-center lg:col-span-7">
			<HeroHeader
				h1Class="max-w-2xl mb-4 text-4xl font-extrabold tracking-tight leading-none md:text-5xl xl:text-6xl dark:text-white"
				pClass="max-w-2xl mb-6 font-light text-gray-500 lg:mb-8 md:text-lg lg:text-xl dark:text-gray-400"
			>
				<svelte:fragment slot="h1">Glyph.rs</svelte:fragment>
				<svelte:fragment slot="paragraph">A text art generator written in Rust.</svelte:fragment>
			</HeroHeader>
		</div>
	</div>
</div>
<!-- 
<div
	class="min-h-[20vw] w-full relative flex flex-col items-center justify-center p-10 border-t-2 border-b-2 border-black dark:border-white"
>
	<div class="w-full mb-10 flex flex-col items-center justify-center text-4xl">
		<div>Examples</div>
	</div>
	<Testimonial>
		<pre class="fira-code leading-tight text-lg font-medium text-gray-900 dark:text-white">
 ██████╗ ██╗     ██╗   ██╗██████╗ ██╗  ██╗   ██████╗ ███████╗
██╔════╝ ██║     ╚██╗ ██╔╝██╔══██╗██║  ██║   ██╔══██╗██╔════╝
██║  ███╗██║      ╚████╔╝ ██████╔╝███████║   ██████╔╝███████╗
██║   ██║██║       ╚██╔╝  ██╔═══╝ ██╔══██║   ██╔══██╗╚════██║
╚██████╔╝███████╗   ██║   ██║     ██║  ██║██╗██║  ██║███████║
 ╚═════╝ ╚══════╝   ╚═╝   ╚═╝     ╚═╝  ╚═╝╚═╝╚═╝  ╚═╝╚══════╝
			</pre>
		<div class="w-full flex flex-col items-center justify-center">
			<div>Shadow</div>
		</div>
	</Testimonial>
</div> -->
