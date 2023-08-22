#include <cstdlib>
#include <limits>
#include <string>
#include <vector>
#include <fstream>
#include <iostream>

class file {
public:
    std::string name;
    size_t size;

    file(std::string fname, size_t size) : name(fname), size(size) {}
};


class Dir {
public:
    Dir* parent;
    std::string name;
    std::vector<Dir*> sub_dirs;
    std::vector<file> files;
    size_t dir_size;

    Dir(std::string name, Dir* parent) : name(name), parent(parent), dir_size(0) {}

    void add_file(std::string file_name, size_t file_size) {
        //std::cout << "\tAttempting to add file: " << file_name << ", size: " << file_size << "...";

        for(int i = 0; i < files.size(); i++) {
            if(files[i].name == file_name) {
                //std::cout << "already exists" << std::endl;
                return;
            }
        }
        //std::cout << "created" << std::endl;
        files.push_back(file(file_name, file_size));
    }

    void add_sub_dir(std::string dir_name) {
        //std::cout << "\tAttempting to add dir: " << dir_name << "...";
        for(int i = 0; i < sub_dirs.size(); i++) {
            if(sub_dirs[i]->name == dir_name) {
                //std::cout << "dir already exists" << std::endl;
                return;
            }
        }
        Dir* new_sub_dir = new Dir(dir_name, this);
        sub_dirs.push_back(new_sub_dir);
        //std::cout << "dir created" << std::endl;
    }

    size_t calc_and_set_dir_size() {
        size_t size = 0;
        for(int i = 0; i < files.size(); i++) {
            size += files[i].size;
        }

        for(int i = 0; i < sub_dirs.size(); i++) {
            size += sub_dirs[i]->calc_and_set_dir_size();
        }
        dir_size = size;

        return size;
    }
};

class FileSystem {
public: 
    Dir* root_dir;
    Dir* curr_dir;
    FileSystem() : root_dir(nullptr), curr_dir(nullptr) {}
    
    void cd(std::string new_dir) {
        //std::cout << "\tcd-ing to " << new_dir << std::endl;

        // first check if we're going back
        if(new_dir == "..") {
            curr_dir = curr_dir->parent;
            //std::cout << "\tcurr_dir set to: " << curr_dir->name << std::endl;
            return;
        } else if(new_dir == "/") {
            curr_dir = root_dir;
            //std::cout << "\tcurr_dir set to: " << curr_dir->name << std::endl;
            return;
        }

        // if the sub directory exists then we change the curr_dir pointer to it
        for(int i = 0; i < curr_dir->sub_dirs.size(); i++) {
            if(curr_dir->sub_dirs[i]->name == new_dir) {
                curr_dir = curr_dir->sub_dirs[i];
                //std::cout << "\tcurr_dir set to: " << curr_dir->name << std::endl;
                return;
            }
        }

        //std::cout << std::endl << std::endl << "CD FAILED!" << std::endl << std::endl;
    }
    
    static size_t part_1_solution(Dir* dir_in) {
        size_t accum = 0;
        for(int i = 0; i < dir_in->sub_dirs.size(); i++) {
            accum += part_1_solution(dir_in->sub_dirs[i]);
        }

        return accum + (dir_in->dir_size <= 100000 ? dir_in->dir_size : 0);
    }

    static void part_2_solution(Dir* dir_in, size_t min_space, size_t* min) {
        if((dir_in->dir_size >= min_space) 
                && (dir_in->dir_size < *min)) {
            *min = dir_in->dir_size;
        }
        
        for(int i = 0; i < dir_in->sub_dirs.size(); i++) {
            FileSystem::part_2_solution(dir_in->sub_dirs[i], min_space, min);
        }
    }
};

int main(void) {
    std::fstream input;
    input.open("input.txt", std::ios::in);
    if(!input.is_open()) {
        std::cout << "Failed to open the file" << std::endl;
        return EXIT_FAILURE;
    }

    FileSystem fs = FileSystem();
    Dir root_dir = Dir("/", nullptr);
    Dir* curr_dir = &root_dir;

    fs.root_dir = &root_dir;
    fs.curr_dir = curr_dir;

    std::vector<std::string> commands;

    std::string line;
    while(std::getline(input, line)) {
        commands.push_back(line);
    }
    input.close();

    // construct the file system from the inputs
    for(int i = 0; i < commands.size(); ) {
        //std::cout << commands[i] << std::endl;
        if(commands[i][0] == '$') {
            if(commands[i][2] == 'c' && commands[i][3] == 'd') {
                fs.cd(commands[i].substr(5, commands[i].length()));
                i++;
            } else if(commands[i][2] == 'l' && commands[i][3] == 's') {
                i++;
                while(i < commands.size() && commands[i][0] != '$') {
                    //std::cout << commands[i] << std::endl;
                    if(std::isdigit(commands[i][0])) { // files
                        size_t tmp_size = std::atoi(commands[i].c_str());
                        size_t name_start = commands[i].find(' ') + 1; // advance to the first character after the space
                        std::string tmp_name = commands[i].substr(name_start, commands[i].length());
                        fs.curr_dir->add_file(tmp_name, tmp_size);
                    } else if(commands[i][0] == 'd' && commands[i][1] == 'i' && commands[i][2] == 'r') {
                        std::string tmp_name = commands[i].substr(4, commands[i].length());
                        fs.curr_dir->add_sub_dir(tmp_name);
                    }
                    i++;
                }
            }
        } else {
            //std::cout << "Sadness" << std::endl;
            return EXIT_FAILURE;
        }
    }

    // should calculate and set directory sizes for the entire file tree
    root_dir.calc_and_set_dir_size();

    size_t used_space = root_dir.dir_size;
    size_t free_space = 70000000 - used_space;
    size_t req_space = 30000000 - free_space;

    size_t min_space = std::numeric_limits<size_t>::max();
    FileSystem::part_2_solution(&root_dir, req_space, &min_space);

    std::cout << "Minimum space: " << min_space << std::endl;

    return EXIT_SUCCESS;
}

