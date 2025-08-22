export default defineNuxtRouteMiddleware((to) => {
  const { isLogged } = useAuth()

  if (!isLogged.value && to.path !== '/login')
    return navigateTo('/login')
})
