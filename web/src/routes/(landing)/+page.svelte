<script lang="ts">
	import Button from "$lib/components/Button.svelte"
	import Link from "$lib/components/Link.svelte"
	import Chevron from "$lib/assets/chevron.svg?raw"
</script>

<svelte:head>
	<title>Lambda</title>
	<meta name="description" content="a lambda deployment tool" />
</svelte:head>

<section class="pt-24 lg:pt-44">
	<div class="flex flex-col gap-y-6">
		<h1
			class="text-center font-manrope text-5xl font-medium -tracking-[1%] text-balance md:text-6xl lg:max-w-xl lg:text-left lg:text-7xl"
		>
			AWS Lambda without the AWS
		</h1>
		<h2 class="text-center text-balance text-white/80 lg:max-w-lg lg:text-left lg:text-lg">
			Universal Lambda deployment tool integrated with your AWS account, for full control and
			no markup.
		</h2>
		<div class="flex items-center justify-center gap-x-6 lg:justify-start">
			<Button class="group duration-300">
				Start now
				{@html Chevron}
			</Button>
			<Link class="duration-300" href="/">Book a demo</Link>
		</div>
	</div>
</section>
