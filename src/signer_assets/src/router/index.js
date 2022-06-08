import { createRouter, createWebHashHistory } from "vue-router";
import Home from "../views/Home.vue";

const routes = [
  {
    path: "/",
    name: "Home",
    component: Home
  },
  // {
  //   path: "/tools",
  //   name: "Tools",
  //   // route level code-splitting
  //   // this generates a separate chunk (tools.[hash].js) for this route
  //   // which is lazy-loaded when the route is visited.
  //   component: () => import("../views/Tools.vue")
  // }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes
});

export default router;
