export const useAlert = () => {
    const visible = ref(false);
    const title = ref("");
    const content = ref("");
    const type = ref<'error' | 'success'>("error");
    function setError(t: string, c: string) {
        type.value = "error";
        title.value = t;
        content.value = c;
        visible.value = true;
    }
    function setSuccess(t: string, c: string) {
        type.value = "success";
        title.value = t;
        content.value = c;
        visible.value = true;
    }
    return { visible, title, content, type, setError, setSuccess };
}
