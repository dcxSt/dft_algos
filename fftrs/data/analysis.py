#!/usr/bin/env python
# coding: utf-8

# In[1]:


import numpy as np
import matplotlib.pyplot as plt


# In[2]:


arr = []
with open("outdc.txt","r") as f:
    for i in range(4): f.readline()
    line = f.readline()
    while line:
        nplus = sum(map(lambda x : 1 if '+' in x else 0, line))
        nminus= sum(map(lambda x : 1 if '-' in x else 0, line))
        idxj = line.index("j")
        if nplus + nminus == 2:
            if nminus == 2:
                idx = 2 + line[2:].index("-")
                real = line[1:idx]
                imag = line[idx:idxj]
            else:
                idx = 2 + line[2:].index("+")
                real = line[1:idx]
                imag = line[idx+1:idxj]
        elif nplus == 1:
            idx = line.index("+")
            real = line[1:idx]
            imag = line[idx+1:idxj]
        elif nminus == 1:
            idx = line.index("-")
            real = line[1:idx]
            imag = line[idx:idxj]
        else:
            raise Exception("Something went wrong")
        c = int(real)+1j*int(imag)
        arr.append(c)
        line=f.readline()


# In[3]:


plt.plot(np.real(arr[1:]),'.',label="real")
plt.plot(np.imag(arr[1:]),'.',label="imag")
plt.legend()
plt.title("18bit 2048-point integer FFT of DC band without 0th term")
plt.savefig("img/18bit_2048-point_DC.png",dpi=300)
plt.show()


# In[4]:


dc = np.ones(2048) * 1000
ft = np.fft.fft(dc)
plt.plot(np.real(ft[1:]),'.',label="real")
plt.plot(np.imag(ft[1:]),'.',label="imag")
plt.show()


# In[ ]:




