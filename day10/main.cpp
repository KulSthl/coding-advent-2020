#include <iostream>
#include <vector>
#include <fstream>
#include <string>
#include <algorithm>

using namespace std;

std::vector<int> read (std::vector<int> v) {
    string line;
    ifstream myfile ("../data.txt");
    if (myfile.is_open())
    {
        while ( getline (myfile,line) )
        {
            v.push_back(std::stoi(line));
        }
        myfile.close();
    }

    else cout << "Unable to open file";

    return v;
}
int sumArray(std::vector<int> v){
    int val = 0;
    for (int j = 0; j < v.size(); ++j) {
        val+=v[j];
    }
    return val;
}
class Difference{
    public:int difference_of_one = 0;
    public:int difference_of_three = 0;
    public:int arangement = 1;
    public:std::vector<int> cache;
};
Difference calc(int num_one,int num_two,Difference difference){
    printf("num_one: %d, num_two: %d -> ",num_one, num_two);
    if((num_two - num_one)== 3){
        difference.difference_of_three+=1;
        printf("Difference of three\n");
    }
    if((num_two - num_one)== 1){
        difference.difference_of_one+=1;
        printf("Difference of one\n");
    }
    return difference;
}
int printArray(std::vector<int> v )
{
        printf("\n[");
    for (int i = 0; i < v.size(); ++i) {
        printf("%d,",v[i]);
    }
        printf("]\n");
}
Difference calc_arrangements(int num_one, int num_two,int num_three, int num_four, Difference difference){
    int l_arrangements = 1;
    if((num_two-num_one)<=3){
        if(difference.cache[num_one]!=0){
            int val = sumArray(difference.cache);
            difference.cache[num_three]=difference.cache[num_one]*val+1;
        }
        else{
            difference.cache[num_three]+=1;
        }
    }
        if((num_three-num_one)<=3){
            if(difference.cache[num_one]!=0){
                int val = sumArray(difference.cache);
                difference.cache[num_three]=difference.cache[num_one]*val+1;
            }
            else{
            difference.cache[num_three]+=1;
            }
        }
        if((num_four-num_one)<=3){
            if(difference.cache[num_one]!=0){
                int val = sumArray(difference.cache);
                difference.cache[num_four]=difference.cache[num_one]*val+1;
            }
            else{
                difference.cache[num_four]+=1;
            }
        }

    return difference;
}
int main()
{
    Difference difference = Difference();
    std::vector<int> v;
    int i;
    v = read(v);
    v.push_back(0);
    std::sort(v.begin(), v.end());
    v.push_back(v[v.size()-1]+3);
    for (int j = 0; j < v[v.size()-1]; ++j) {
        difference.cache.push_back(0);
    }
    for(int i = 0; i<v.size();++i){
        printf("%d\n",v[i]);
        if (i>0){
            difference = calc(v[i-1],v[i],difference);
            if(i < v.size()-3){
                difference = calc_arrangements(v[i-1],v[i],v[i+1],v[i+2],difference);
            }
        }
    }
    printf("1dif: %d,3dif: %d\n Result = %d",difference.difference_of_one,difference.difference_of_three,difference.difference_of_one*difference.difference_of_three);
}

