#include<stdio.h>
#include<pthread.h>

void *simple_thread01(){
    printf("The thread 01!\n");
    return NULL;
}

void *simple_thread02(){
    printf("The thread 02!\n");
    return NULL;
}

int main(){

    pthread_t th1;
    pthread_t th2;
    // unsigned int th1;
    // unsigned int th2;

    pthread_create(&th1, NULL, simple_thread01, NULL);
    pthread_create(&th2, NULL, simple_thread02, NULL);

    pthread_join(th1, NULL);
    pthread_join(th2, NULL);
    
    puts("Main running,\n");

    return 0;
}