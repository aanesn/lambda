<script lang="ts">
	import type { LayoutProps } from "./$types"
	import { goto } from "$app/navigation"

	let { data, children }: LayoutProps = $props()

	$effect(() => {
		!data.user && goto("/", { replaceState: true })
	})
</script>

{@render children()}
