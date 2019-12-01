<template>
  <div class="wrapper">
    <nav id="sidebar" :class="{ active: sidebarActive }">
      <div class="sidebar-logo">
        <router-link :to="{ name: 'Home' }" class="logo">
          Lucid ᵏᵛ
        </router-link>
        <b-badge v-if="endpoint.version" :href="`https://github.com/lucid-kv/lucid/releases/tag/${endpoint.version}`" target="_blank" rel="noopener" class="version" pill variant="dark">v{{ endpoint.version }}</b-badge>
      </div>

      <ul class="list-unstyled components">
        <template v-if="!isLoggedIn">
          <a class="list-group-item list-group-item-action heading">Log in</a>

          <router-link :to="{ name: 'Login' }" exact-active-class="active" class="list-group-item list-group-item-action d-flex justify-content-between align-items-center">
            <feather type="log-in" size="1rem" />
            <span class="mr-auto">Log in</span>
          </router-link>
        </template>

        <template v-else>
          <a class="list-group-item list-group-item-action heading">Overview</a>

          <router-link :to="{ name: 'Home' }" exact-active-class="active" class="list-group-item list-group-item-action d-flex justify-content-between align-items-center">
            <feather type="home" size="1rem" />
            <span class="mr-auto">Dashboard</span>
          </router-link>

          <a class="list-group-item list-group-item-action heading">Node</a>

          <router-link :to="{ name: 'KvProofOfConcept' }" exact-active-class="active" class="list-group-item list-group-item-action d-flex justify-content-between align-items-center">
            <feather type="database" class="text-info" size="1rem" />
            <span class="mr-auto">Data storage</span>
          </router-link>
          <router-link to="/soon" exact-active-class="active" class="list-group-item list-group-item-action d-flex justify-content-between align-items-center disabled text-muted">
            <feather type="key" class="text-muted text-success" size="1rem" />
            <span class="mr-auto">Tokens issuing</span>
          </router-link>
          <router-link to="/soon" exact-active-class="active" class="list-group-item list-group-item-action d-flex justify-content-between align-items-center disabled text-muted">
            <feather type="file-text" class="text-muted text-warning" size="1rem" />
            <span class="mr-auto">Live logging</span>
          </router-link>
          <router-link to="/soon" exact-active-class="active" class="list-group-item list-group-item-action d-flex justify-content-between align-items-center disabled text-muted">
            <feather type="settings" class="text-muted text-danger" size="1rem" />
            <span class="mr-auto">Node settings</span>
          </router-link>

          <a class="list-group-item list-group-item-action heading">Features</a>

          <router-link to="/soon" exact-active-class="active" class="list-group-item list-group-item-action d-flex justify-content-between align-items-center disabled text-muted">
            <feather type="layers" size="1rem" />
            <span class="mr-auto">Distributed</span>
          </router-link>
          <router-link to="/soon" exact-active-class="active" class="list-group-item list-group-item-action d-flex justify-content-between align-items-center disabled text-muted">
            <feather type="activity" size="1rem" />
            <span class="mr-auto">Monitoring</span>
          </router-link>
          <router-link to="/soon" exact-active-class="active" class="list-group-item list-group-item-action d-flex justify-content-between align-items-center disabled text-muted">
            <feather type="shield" size="1rem" />
            <span class="mr-auto">ACLs / CORS</span>
          </router-link>
          <router-link to="/soon" exact-active-class="active" class="list-group-item list-group-item-action d-flex justify-content-between align-items-center disabled text-muted">
            <feather type="terminal" size="1rem" />
            <span class="mr-auto">Command Line</span>
          </router-link>
          <router-link to="/soon" exact-active-class="active" class="list-group-item list-group-item-action d-flex justify-content-between align-items-center disabled text-muted">
            <feather type="power" size="1rem" />
            <span class="mr-auto">Stop / Restart</span>
          </router-link>

          <a class="list-group-item list-group-item-action heading">Log out</a>

          <a @click="logOut" class="list-group-item list-group-item-action d-flex justify-content-between align-items-center pointer">
            <feather type="log-out" size="1rem" />
            <span class="mr-auto">Log out</span>
          </a>
        </template>

        <a class="list-group-item list-group-item-action heading">Other</a>

        <a href="https://github.com/lucid-kv/lucid" target="_blank" rel="noopener" class="list-group-item list-group-item-action d-flex justify-content-between align-items-center">
          <feather type="github" size="1rem" />
          <span class="mr-auto">Github Repository</span>
        </a>
        <a href="https://docs.lucid-kv.store" target="_blank" rel="noopener" class="list-group-item list-group-item-action d-flex justify-content-between align-items-center">
          <feather type="book-open" size="1rem" />
          <span class="mr-auto">Lucid Documentation</span>
        </a>
      </ul>
    </nav>

    <div id="content" class="container-fluid">
      <b-button variant="dark" @click="sidebarActive = !sidebarActive" type="button" id="sidebarCollapse" class="sidebar-toggler">
        <feather type="menu" size="1rem" />
      </b-button>

      <slot />
    </div>
  </div>
</template>

<script>
import { mapActions, mapGetters, mapState } from 'vuex'

export default {
  data() {
    return {
      sidebarActive: false
    }
  },
  computed: {
    ...mapGetters(['isLoggedIn']),
    ...mapState(['endpoint'])
  },
  methods: {
    ...mapActions(['logOut'])
  }
}
</script>

<style scoped>
.wrapper {
  display: flex;
  width: 100%;
  align-items: stretch;
  height: 100vh;
  max-height: 100vh;
  overflow-y: hidden;
}

a, a:hover, a:focus {
  color: inherit;
  text-decoration: none;
  transition: all 0.3s;
}

#content {
  transition: all 0.3s;
  overflow-y: auto;
  margin: 0;
  height: 100vh;
}

.navbar {
  background: #fff;
  border: none;
  border-radius: 0;
  margin-bottom: 40px;
  box-shadow: 1px 1px 3px rgba(0, 0, 0, 0.1);
}

#sidebar {
  color: #fff;
  background: #161d33;
  padding-top: 10px;
  overflow-x: hidden;
  overflow-y: scroll;
  transition-duration: 1s;
  height: 100vh;
  min-width: 250px;
}

#sidebar.active {
  margin-left: -250px;
}

#sidebar::-webkit-scrollbar {
  width: 7px !important;
  background: none;
}

#sidebar::-webkit-scrollbar-thumb {
  background-color: #232d469d !important;
  border-radius: 5px !important;
}

#sidebar .sidebar-logo {
  padding: 10px;
  font-weight: 500;
  font-size: 18px;
  color: #ffffff;
  line-height: 1.2;
  text-align: center;
}
#sidebar .sidebar-logo .logo {
  display: block;
  margin-left: 17px;
}
#sidebar .sidebar-logo .version {
  display: block;
  font-size: 11px;
  margin: 0 auto;
  margin-top: 9px;
  width: fit-content;
}

#sidebar .list-group-item-action .feather {
  margin-right: 18px;
  vertical-align: top;
  opacity: 0.7;
  width: 16px;
  height: 16px;
  vertical-align: text-bottom;
}

.list-group-item-action {
  width: auto;
}

#sidebar .list-group-item-action.heading {
  color: #838aa0;
  font-size: 12px;
  padding-bottom: 5px;
}

#sidebar .list-group, #sidebar .list-group-item {
  background: transparent;
  box-shadow: none;
  border: 0pt none;
}

#sidebar .list-group-item-action {
  line-height: 1.125rem;
  color: #b9c0d3;
  font-size: 14px;
  font-weight: 400;
}

#sidebar a {
  display: block;
  border-radius: 5px;
  margin: 5px 10px;
}

#sidebar a:not(.heading):not(.logo):hover {
  color: #fff;
  background: #323b5c;
}

#sidebar a.active {
  color: #fff;
  background: #27304d;
}

.sidebar-toggler {
  position: absolute;
  right: 15px;
  display: flex;
  justify-content: center;
  align-items: center;
}

@media (max-width: 768px) {
  #sidebar {
    margin-left: -250px;
  }
  #sidebar.active {
    margin-left: 0;
  }
}

@media (min-width: 768px) {
  .sidebar-toggler {
    display:none !important;
  }
}
</style>
