<script lang="ts">
	import Button from "$lib/components/Button.svelte"
	import Link from "$lib/components/Link.svelte"
</script>

<svelte:head>
	<title>Lambda</title>
	<meta name="description" content="a lambda deployment tool" />
</svelte:head>

<section class="pt-24 lg:pt-44">
	<div class="flex flex-col gap-y-6">
		<h1
			class="max-w-xl font-manrope text-5xl font-medium -tracking-[1%] text-balance md:text-6xl lg:text-7xl"
		>
			AWS Lambda without the AWS
		</h1>
		<h2 class="w-11/12 max-w-lg text-white/80 lg:text-lg">
			Universal Lambda deployment tool integrated with your AWS account, for full control and
			no markup.
		</h2>
		<div class="flex items-center gap-x-4">
			<Button class="duration-300">Start now</Button>
			<Link href="/" class="duration-300">Book a demo</Link>
		</div>
	</div>
</section>
