<script>
	import Button from "$lib/components/ui/Button.svelte"
	import Link from "$lib/components/ui/Link.svelte"
	import Lambdas from "$lib/webgl/Lambdas.svelte"
</script>

<svelte:head>
	<title>Lambda</title>
	<meta name="description" content="a lambda deployment tool" />
</svelte:head>

<Lambdas />
<main>
	<section class="pt-24 lg:pt-44">
		<div class="flex flex-col items-center gap-y-6 lg:gap-y-8">
			<h1
				class="text-center text-5xl font-semibold leading-[1.1] tracking-[-0.4px] md:text-6xl md:leading-none lg:text-7xl xl:text-[80px]"
			>
				Lambda is finally<br class="hidden min-[400px]:flex" /> effortless.
			</h1>
			<h2 class="max-w-sm text-center lg:max-w-md lg:text-lg">
				No plugins or libraries. Deploy Lambda functions using your favorite web framework.
			</h2>
			<div class="flex items-center gap-x-6">
				<Button class="duration-300">Start now</Button>
				<Link class="duration-300" href="/">Docs</Link>
			</div>
		</div>
	</section>
</main>
