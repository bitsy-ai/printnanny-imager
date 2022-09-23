import { useRouter } from "vue-router";
import { useStore } from "@/store";
import { AlertAction, UiAlert } from "@/types";
import { ExclamationTriangleIcon } from "@heroicons/vue/24/outline";

const ResetAction = {
  color: "indigo",
  text: "Reset",
  onClick: () => {
    const store = useStore();
    const router = useRouter();
    store.$reset();
    router.push({ name: "select-image" });
  },
} as AlertAction;

function showError(e: Error) {
  console.error(e);
  // handle actione rror
  const store = useStore();
  const message = e.toString();
  const header = "Error";

  const alert = {
    message,
    header,
    icon: ExclamationTriangleIcon,
    actions: [ResetAction],
  } as UiAlert;

  store.showError(alert);
}

export { showError, ResetAction };
