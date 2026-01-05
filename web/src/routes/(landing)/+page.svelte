<script lang="ts">
	import Button from "$lib/components/Button.svelte"
	import Link from "$lib/components/Link.svelte"
</script>

<svelte:head>
	<title>Lambda.new</title>
	<meta name="description" content="A Lambda deployment tool" />
</svelte:head>

<div class="py-20 lg:py-40">
	<div class="flex flex-col items-center justify-center gap-y-4 text-center">
		<h1 class="font-manrope text-4xl font-semibold md:text-5xl lg:text-6xl">
			Lambda is finally <br /> effortless.
		</h1>
		<h2 class="max-w-md text-white/70 lg:text-lg">
			Deploy Lambda functions using your favorite web framework without wrestling with the
			console.
		</h2>
		<div class="flex items-center gap-x-4">
			<Button class="duration-300" href="/login">Start now</Button>
			<Link class="duration-300" href="/docs">Documentation</Link>
		</div>
	</div>
</div>
