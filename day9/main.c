#include <stdio.h>
#include <stdlib.h>
int currentPos = 0;
const int length = 25;
struct element *list;
struct element
{
    int value;            /* der Wert des Elements          */
    struct element *next; /* Zeiger auf das nächste Element */
};
void printArray(int array[],int length){
    printf("array is\n");
    for (int c = 0; c < length; c++)
        printf("%d\n", array[c]);
}
 int init(const struct element *e,int preamble[], int pointer){
    int counter = 0;
    for(int i = 0; i<length;i++){
        preamble[i] = (int)e->value;
        e = e->next;
    }
    pointer = length;
}
int sort_insert(int preamble[],int value){
    for (int i = 0; i < length; ++i) {
        if(i+1 < length){
            int old_val = preamble[i+1];
            preamble[i] = old_val;
        }
    }
    preamble[length-1] = value;
    return 1;
}
int add_new(const struct element *e,int preamble[], int pointer,int list_length){
    if(pointer < list_length){
        int value = e->value;
        sort_insert(preamble,value);
        return pointer + 1;
    }
}

void append(struct element **lst, int value)
{
    struct element *neuesElement;
    /* Zeiger auf die Einfügeposition ermitteln, d.h. bis zum Ende laufen */
    while( *lst != NULL )
    {
        lst = &(*lst)->next;
    }
    neuesElement = malloc(sizeof(*neuesElement)); /* erzeuge ein neues Element */
    neuesElement->value = value;
    neuesElement->next = NULL; /* Wichtig für das Erkennen des Listenendes     */
    *lst = neuesElement;
}
int isInPreamble(int target,int preamble[]){
//    printArray(preamble);
    for (int i = 0; i < length; ++i) {
            int firstNumber = preamble[i];
            for (int j = 0; j < length; ++j) {
                int secondNumber = preamble[j];
                if(firstNumber != secondNumber){
                    if(target == (firstNumber + secondNumber)){
                        return 1;
                    }
                }
            }
    }
    return 0;
}
int calc_low_high_values_in_range(int array[],int lidx, int hidx){
    int lowestValue = 999999999;
    int highestValue = 0;
    for (int i = lidx; i < hidx+1 ; ++i) {
        int value = array[i];
        if(lowestValue > value){
            lowestValue = value;
        }
        if(highestValue < value){
            highestValue  = value;
        }
    }
    printf("Low = %d, High = %d\nResult += %d",lowestValue,highestValue,highestValue+lowestValue);
}
int calc_to_target(const struct element *e,int target,int list_length){
     int e_to_array[list_length];
    for (int i = 0; i < list_length; ++i) {
        int value = e->value;
        e_to_array[i] = value;
        e = e->next;
    }
    int lowest_index;
    int highest_index;
    for (int i = 0; i < list_length; ++i) {
        int firstNumber = e_to_array[i];
        int secondNumber = 0;
        if(firstNumber < target){
            for (int j = i+1; j < list_length; ++j) {
                if(e_to_array[j]<target && e_to_array[j]>0){
                    secondNumber+=e_to_array[j];
                    if(secondNumber<target && secondNumber > 0){
                         if(target == (secondNumber+firstNumber)){
                             lowest_index = i;
                             highest_index = j;
                             printf("%d,%d\n",e_to_array[i],e_to_array[j]);
                             calc_low_high_values_in_range(e_to_array,lowest_index,highest_index);
                             return 1;
                         }
                    }
                    else{
                        break;
                    }
                }
            }
        }
    }
 }

int get_weakness(const struct element *e,int preamble[], int pointer,int list_length)
{
    int l_pointer = pointer;
    for(int i = 0 ;i < length; i++){
        e = e->next;
    }
    for( ; e != NULL ; e = e->next )
    {
        int value = e->value;
        int result = isInPreamble(value,preamble);
        if(result == 1){
            l_pointer = add_new(e,preamble,l_pointer,list_length);
        }
        else{
        printf("%d\n", value);
        return value;
        }
    }
}
int read(){
    int counter = 0;
    FILE *fp; // declaration of file pointer
    char con[1000]; // variable to read the content
    fp =fopen("../data.txt","r");// opening of file
    if (!fp)// checking for error
        return 1;
    while (fgets(con,1000, fp)!=NULL)// reading file content
    {
        int i;
        sscanf(con, "%d", &i);
        append(&list, i);
        counter += 1;
    }
    fclose(fp); // closing file
    return counter;
}
int main() {
    int pointer = length+1;
    int preamble[length];
    printf("Hello, World!\n");
    int list_length = read();
    init(list,preamble,pointer);

    int weakness = get_weakness(list,preamble,pointer,list_length);
//    read();
    calc_to_target(list,weakness,list_length);


//    printArray(preamble);
    return 0;
}

