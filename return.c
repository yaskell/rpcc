int main(void) {
    int a = 0;
    {
        int a = 1;
    }
    {
        int a = 2;
        {
            int a = 3;
        }
    }
    return a;
}
