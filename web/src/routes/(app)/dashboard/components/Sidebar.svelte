<script lang="ts">
	import * as Sidebar from "$lib/components/sidebar"
	import * as DropdownMenu from "$lib/components/dropdown-menu"
	import * as Avatar from "$lib/components/avatar"
	import Projects from "$lib/assets/projects.svg?raw"
	import Domains from "$lib/assets/domains.svg?raw"
	import Analytics from "$lib/assets/analytics.svg?raw"
	import ChevronsUpDown from "$lib/assets/chevrons-up-down.svg?raw"
	import Logout from "$lib/assets/logout.svg?raw"
	import PlusFilled from "$lib/assets/plus-filled.svg?raw"
	import Plus from "$lib/assets/plus.svg?raw"
	import Logomark from "$lib/assets/logomark.svg?raw"
	import { page } from "$app/state"
	import type { User } from "$lib/bindings"
	import { apiUrl } from "$lib/utils"

	let { user }: { user: User | null } = $props()

	const navItems = [
		{
			title: "New project",
			href: "/dashboard/new-project",
			icon: PlusFilled
		},
		{
			title: "Projects",
			href: "/dashboard/projects",
			icon: Projects
		},
		{
			title: "Domains",
			href: "/dashboard/domains",
			icon: Domains
		},
		{
			title: "Analytics",
			href: "/dashboard/analytics",
			icon: Analytics
		}
	]

	const userDropdownMenuItems = [
		{
			title: "Log out",
			href: `${apiUrl}/auth/logout`,
			icon: Logout
		}
	]

	const teamDropdownMenuItems = [
		{
			title: "Add team",
			href: "##",
			icon: Plus
		}
	]

	const sidebar = Sidebar.useSidebar()
</script>

<Sidebar.Root>
	<Sidebar.Header>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						{#snippet child({ props })}
							<Sidebar.MenuButton
								{...props}
								intent="secondary"
								size="lg"
								class="group-data-[state=collapsed]:p-1.5!"
							>
								{@html Logomark}
								<div class="grid flex-1 text-start text-sm leading-tight">
									<span class="truncate font-medium">Lambda.new</span>
									<span class="truncate text-xs">Free plan</span>
								</div>
								{@html ChevronsUpDown}
							</Sidebar.MenuButton>
						{/snippet}
					</DropdownMenu.Trigger>
					<DropdownMenu.Content
						class="w-(--bits-dropdown-menu-anchor-width)"
						side={sidebar.isMobile ? "bottom" : "right"}
					>
						{#each teamDropdownMenuItems as { title, href, icon }}
							<DropdownMenu.Item>
								{#snippet child({ props })}
									<a {...props} {href}>
										{@html icon}
										<span>{title}</span>
									</a>
								{/snippet}
							</DropdownMenu.Item>
						{/each}
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Header>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.Menu>
				{#each navItems as { title, href, icon }}
					<Sidebar.MenuItem>
						<Sidebar.MenuButton
							tooltipContent={title}
							isActive={page.url.pathname.startsWith(href)}
						>
							{#snippet child({ props })}
								<a {...props} {href}>
									{@html icon}
									<span>{title}</span>
								</a>
							{/snippet}
						</Sidebar.MenuButton>
					</Sidebar.MenuItem>
				{/each}
			</Sidebar.Menu>
		</Sidebar.Group>
	</Sidebar.Content>
	<Sidebar.Footer>
		<Sidebar.Menu>
			<Sidebar.MenuItem>
				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						{#snippet child({ props })}
							<Sidebar.MenuButton intent="secondary" size="lg" {...props}>
								<Avatar.Root class="size-8">
									<Avatar.Image src={user?.avatar_url} alt="avatar" />
									<Avatar.Fallback>
										{user?.name.charAt(0).toUpperCase()}
									</Avatar.Fallback>
								</Avatar.Root>
								<div class="grid flex-1 text-start text-sm leading-tight">
									<span class="truncate font-medium">{user?.name}</span>
									<span class="truncate text-xs">{user?.email}</span>
								</div>
								{@html ChevronsUpDown}
							</Sidebar.MenuButton>
						{/snippet}
					</DropdownMenu.Trigger>
					<DropdownMenu.Content class="w-(--bits-dropdown-menu-anchor-width)" side="top">
						{#each userDropdownMenuItems as { title, href, icon }}
							<DropdownMenu.Item>
								{#snippet child({ props })}
									<a {...props} {href}>
										{@html icon}
										<span>{title}</span>
									</a>
								{/snippet}
							</DropdownMenu.Item>
						{/each}
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</Sidebar.MenuItem>
		</Sidebar.Menu>
	</Sidebar.Footer>
	<Sidebar.Rail />
</Sidebar.Root>
