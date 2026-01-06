<script lang="ts">
	import Separator from "$lib/components/Separator.svelte"
	import * as Sidebar from "$lib/components/sidebar"
	import * as Breadcrumb from "$lib/components/breadcrumb"
	import { page } from "$app/state"

	let segments = $derived(page.url.pathname.split("/").filter(Boolean).slice(1))
</script>

<header
	class="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[state=collapsed]/sidebar-wrapper:h-12"
>
	<div class="flex items-center gap-2 px-4">
		<Sidebar.Trigger />
		<Separator orientation="vertical" class="me-2 data-[orientation=vertical]:h-4" />
		<Breadcrumb.List>
			{#each segments as segment, i}
				{@const title =
					segment.charAt(0).toUpperCase() + segment.slice(1).replaceAll("-", " ")}
				<Breadcrumb.Item>
					{#if i === segments.length - 1}
						<Breadcrumb.Page>{title}</Breadcrumb.Page>
					{:else}
						<Breadcrumb.Link href={`/dashboard/${segments.slice(0, i + 1).join("/")}`}>
							{title}
						</Breadcrumb.Link>
					{/if}
				</Breadcrumb.Item>
				{#if i < segments.length - 1}
					<Breadcrumb.Separator />
				{/if}
			{/each}
		</Breadcrumb.List>
	</div>
</header>
